import tkinter as tk
from tkinter import messagebox, ttk


class EditableTable(ttk.LabelFrame):
    """
    Editable table based on ttk.Treeview.

    Editing logic:
    - double click a cell to edit it directly;
    - Enter saves;
    - focus out saves;
    - Escape cancels;
    - Add row creates an empty row directly in the table.
    """

    def __init__(
        self,
        parent,
        title: str,
        columns: list[tuple[str, str, int]],
        height: int = 7,
        header_image=None,
    ):
        super().__init__(parent, text=title, padding=8)
        self.columns = columns
        self.column_keys = [col[0] for col in columns]

        self.active_editor = None
        self._editing_item = None
        self._editing_column = None

        # Layout:
        # row 0 = header (image + legend)
        # row 1 = treeview (expands)
        # row 2 = buttons (always visible)
        
        self.columnconfigure(0, weight=1)

        current_row = 0

        # -------------------------------------------------------------
        # Header image + legend
        # -------------------------------------------------------------
        if header_image is not None:

            header_frame = ttk.Frame(self)

            image_label = ttk.Label(
                header_frame,
                image=header_image,
            )
            image_label.image = header_image
            image_label.pack(side="left", anchor="n")

            legend_text = (
                "Hs = Stick length [mm]\n"
                "As = Stick width [mm]\n"
                "Ss = Stick thickness [mm]\n"
                "Bs = Fin length [mm]"
            )

            ttk.Label(
                header_frame,
                text=legend_text,
                justify="left",
            ).pack(
                side="left",
                padx=20,
                anchor="n",
            )

            header_frame.grid(
                row=current_row,
                column=0,
                sticky="ew",
                pady=(0, 8),
            )

            current_row += 1

        # -------------------------------------------------------------
        # Treeview area (expandable)
        # -------------------------------------------------------------
        tree_row = current_row

        tree_frame = ttk.Frame(self)
        tree_frame.grid(
            row=tree_row,
            column=0,
            sticky="nsew",
        )

        # Deve espandersi e comprimersi la riga della tabella,
        # non quella dei pulsanti.
        self.rowconfigure(tree_row, weight=1)

        tree_frame.columnconfigure(0, weight=1)
        tree_frame.rowconfigure(0, weight=1)

        self.tree = ttk.Treeview(
            tree_frame,
            columns=self.column_keys,
            show="headings",
            height=height,
        )

        for key, heading, width in columns:
            self.tree.heading(key, text=heading)

            self.tree.column(
                key,
                width=width,
                anchor="center",
                stretch=True,
            )

        self.tree.grid(
            row=0,
            column=0,
            sticky="nsew",
        )

        current_row += 1

        # -------------------------------------------------------------
        # Buttons (always visible)
        # -------------------------------------------------------------
        buttons = ttk.Frame(self)

        buttons.grid(
            row=current_row,
            column=0,
            sticky="ew",
            pady=(6, 0),
        )

        ttk.Button(
            buttons,
            text="Add row",
            command=self.add_blank_row,
        ).pack(side="left")

        ttk.Button(
            buttons,
            text="Remove selected",
            command=self.remove_selected,
        ).pack(side="left", padx=(6, 0))

        ttk.Label(
            buttons,
            text="Double-click a cell to edit",
        ).pack(side="left", padx=(12, 0))

        # -------------------------------------------------------------
        # Bindings
        # -------------------------------------------------------------
        self.tree.bind("<Double-1>", self._start_cell_edit)
        self.tree.bind("<Button-1>", self._handle_single_click)

    # ------------------------------------------------------------------
    # Public API used by app.py
    # ------------------------------------------------------------------

    def add_blank_row(self):
        empty_values = tuple("" for _ in self.column_keys)

        item_id = self.tree.insert(
            "",
            "end",
            values=empty_values,
        )

        self.tree.selection_set(item_id)
        self.tree.focus(item_id)

        self.after(
            50,
            lambda: self._edit_cell(item_id, "#1"),
        )

    def remove_selected(self):
        self._destroy_active_editor(save=False)

        selected = self.tree.selection()

        if not selected:
            messagebox.showinfo(
                "No selection",
                "Select one or more rows to remove.",
            )
            return

        for item in selected:
            self.tree.delete(item)

    def set_rows(self, rows: list[tuple]) -> None:
        self.clear()

        for row in rows:
            self.tree.insert("", "end", values=row)

    def get_rows(self) -> list:
        rows = []

        for item in self.tree.get_children():
            rows.append(tuple(self.tree.item(item, "values")))

        return rows

    def clear(self) -> None:
        self._destroy_active_editor(save=False)

        for item in self.tree.get_children():
            self.tree.delete(item)

    # ------------------------------------------------------------------
    # Direct cell editing
    # ------------------------------------------------------------------

    def _handle_single_click(self, _event):
        if self.active_editor is not None:
            self._destroy_active_editor(save=True)

    def _start_cell_edit(self, event):
        region = self.tree.identify("region", event.x, event.y)

        if region != "cell":
            return

        item_id = self.tree.identify_row(event.y)
        column = self.tree.identify_column(event.x)

        if item_id and column:
            self._edit_cell(item_id, column)
        
    def _edit_cell(self, item_id, column):

        self._destroy_active_editor(save=True)

        bbox = self.tree.bbox(item_id, column)

        if not bbox:
            return

        x, y, width, height = bbox

        values = self.tree.item(item_id, "values")

        col_index = int(column.replace("#", "")) - 1

        current_value = ""

        if col_index < len(values):
            current_value = values[col_index]

        editor = ttk.Entry(self.tree)

        editor.insert(0, current_value)

        editor.place(
            x=x,
            y=y,
            width=width,
            height=height,
        )

        editor.focus_set()
        editor.select_range(0, tk.END)

        self.active_editor = editor
        self._editing_item = item_id
        self._editing_column = column

        editor.bind(
            "<Return>",
            lambda e: self._destroy_active_editor(save=True)
        )

        editor.bind(
            "<Escape>",
            lambda e: self._destroy_active_editor(save=False)
        )

        editor.bind(
            "<FocusOut>",
            lambda e: self._destroy_active_editor(save=True)
        )

    def _destroy_active_editor(self, save: bool = True) -> None:
        """
        Close current inline editor safely.

        save=True: commit value to cell before closing.
        save=False: discard current edit.
        """

        editor = getattr(self, "active_editor", None)

        if editor is None:
            return

        item_id = getattr(self, "_editing_item", None)
        column = getattr(self, "_editing_column", None)

        if save and item_id and column:
            try:
                new_value = editor.get()

                values = list(
                    self.tree.item(item_id, "values")
                )

                col_index = int(
                    column.replace("#", "")
                ) - 1

                if 0 <= col_index < len(values):
                    values[col_index] = new_value

                    self.tree.item(
                        item_id,
                        values=values,
                    )

            except Exception:
                pass

        try:
            editor.destroy()
        except Exception:
            pass

        self.active_editor = None
        self._editing_item = None
        self._editing_column = None
        
