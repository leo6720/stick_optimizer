import tkinter as tk
from tkinter import messagebox, ttk


class HierarchicalInputTable(ttk.Frame):
    """
    Hierarchical input for Stick Types and their Formats.
    """

    def __init__(
        self,
        parent,
        title: str,
        header_image=None,
    ):
        super().__init__(parent, style="Sidebar.TFrame", padding=12)
        
        self.active_editor = None
        self._editing_item = None
        self._editing_column = None

        self.columnconfigure(0, weight=1)
        current_row = 0

        title_lbl = ttk.Label(
            self,
            text=title,
            style="SidebarHeader.TLabel",
        )
        title_lbl.grid(row=current_row, column=0, sticky="w", pady=(0, 8))
        current_row += 1

        if header_image is not None:
            header_frame = ttk.Frame(self, style="Sidebar.TFrame")
            image_label = ttk.Label(header_frame, image=header_image, style="Sidebar.TLabel")
            image_label.image = header_image
            image_label.pack(side="left", anchor="n")

            legend_text = (
                "Hs = Lunghezza stick [mm]\n"
                "As = Larghezza stick [mm]\n"
                "Ss = Spessore stick [mm]\n"
                "Bs = Lunghezza pinna [mm]"
            )
            ttk.Label(header_frame, text=legend_text, justify="left", style="Sidebar.TLabel").pack(side="left", padx=20, anchor="n")
            header_frame.grid(row=current_row, column=0, sticky="ew", pady=(0, 8))
            current_row += 1

        tree_frame = ttk.Frame(self)
        tree_frame.grid(row=current_row, column=0, sticky="nsew")
        self.rowconfigure(current_row, weight=1)
        tree_frame.columnconfigure(0, weight=1)
        tree_frame.rowconfigure(0, weight=1)

        self.columns = [
            ("name", "Nome / Conteggio", 105),
            ("hs", "Hs", 50),
            ("as", "As", 50),
            ("ss", "Ss", 50),
            ("bs", "Bs", 50),
        ]
        self.column_keys = [c[0] for c in self.columns]

        self.tree = ttk.Treeview(
            tree_frame,
            columns=self.column_keys[1:],
            show="tree headings",
            height=12,
        )

        self.tree.heading("#0", text="Impilamenti")
        self.tree.column("#0", width=115)

        for key, heading, width in self.columns[1:]:
            self.tree.heading(key, text=heading)
            self.tree.column(key, width=width, anchor="center")

        self.tree.grid(row=0, column=0, sticky="nsew")
        current_row += 1

        buttons = ttk.Frame(self, style="Sidebar.TFrame")
        buttons.grid(row=current_row, column=0, sticky="ew", pady=(6, 0))

        ttk.Button(buttons, text="Agg. Stick", command=self.add_stick).pack(side="left")
        ttk.Button(buttons, text="Agg. Formato", command=self.add_format).pack(side="left", padx=4)

        self.tree.bind("<Double-1>", self._start_cell_edit)
        self.tree.bind("<Button-1>", self._handle_single_click)
        self.tree.bind("<Button-3>", self._show_context_menu)

    def _show_context_menu(self, event):
        item = self.tree.identify_row(event.y)
        if item:
            self.tree.selection_set(item)
            menu = tk.Menu(self, tearoff=0)
            menu.add_command(label="Rimuovi", command=self.remove_selected)
            menu.post(event.x_root, event.y_root)

    def add_stick(self, name="Nuovo Stick"):
        item_id = self.tree.insert("", "end", text=name, values=("", "", "", ""), open=True)
        self.tree.selection_set(item_id)
        self.tree.see(item_id)
        return item_id

    def add_format(self, parent_id=None, count="10"):
        if parent_id is None:
            selected = self.tree.selection()
            if not selected:
                messagebox.showwarning("Attenzione", "Seleziona uno Stick per aggiungere un formato.")
                return
            parent_id = selected[0]
            if self.tree.parent(parent_id) != "":
                parent_id = self.tree.parent(parent_id)

        item_id = self.tree.insert(parent_id, "end", text=count, values=("", "", "", ""))
        self.tree.item(parent_id, open=True)
        self.tree.selection_set(item_id)
        self.tree.see(item_id)
        return item_id

    def remove_selected(self):
        self._destroy_active_editor(save=False)
        selected = self.tree.selection()
        for item in selected:
            self.tree.delete(item)

    def clear(self):
        self._destroy_active_editor(save=False)
        for item in self.tree.get_children():
            self.tree.delete(item)

    def _handle_single_click(self, _event):
        if self.active_editor is not None:
            self._destroy_active_editor(save=True)

    def _start_cell_edit(self, event):
        region = self.tree.identify("region", event.x, event.y)
        if region not in ("tree", "cell"):
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
        
        # If it's the tree column (#0), we edit the item text (Stick Name or Count)
        if column == "#0":
            current_value = self.tree.item(item_id, "text")
        else:
            col_index = int(column.replace("#", "")) - 1
            values = self.tree.item(item_id, "values")
            current_value = values[col_index] if col_index < len(values) else ""
            
            # Formats (children) don't have dimensions, they inherit from parent Stick
            if self.tree.parent(item_id) != "":
                return

        editor = ttk.Entry(self.tree)
        editor.insert(0, current_value)
        editor.place(x=x, y=y, width=width, height=height)
        editor.focus_set()
        editor.select_range(0, tk.END)

        self.active_editor = editor
        self._editing_item = item_id
        self._editing_column = column

        editor.bind("<Return>", lambda e: self._destroy_active_editor(save=True))
        editor.bind("<Escape>", lambda e: self._destroy_active_editor(save=False))
        editor.bind("<FocusOut>", lambda e: self._destroy_active_editor(save=True))

    def _destroy_active_editor(self, save: bool = True) -> None:
        editor = getattr(self, "active_editor", None)
        if editor is None: return
        item_id = getattr(self, "_editing_item", None)
        column = getattr(self, "_editing_column", None)

        if save and item_id and column:
            new_val = editor.get()
            if column == "#0":
                self.tree.item(item_id, text=new_val)
            else:
                values = list(self.tree.item(item_id, "values"))
                col_index = int(column.replace("#", "")) - 1
                if 0 <= col_index < len(values):
                    values[col_index] = new_val
                    self.tree.item(item_id, values=values)

        editor.destroy()
        self.active_editor = None


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
        combobox_columns: dict = None,
    ):
        super().__init__(parent, text=title, padding=8)
        self.columns = columns
        self.column_keys = [col[0] for col in columns]
        self.combobox_columns = combobox_columns or {}

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
                "Hs = Lunghezza stick [mm]\n"
                "As = Larghezza stick [mm]\n"
                "Ss = Spessore stick [mm]\n"
                "Bs = Lunghezza pinna [mm]"
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

        col_key = self.column_keys[col_index] if 0 <= col_index < len(self.column_keys) else None
        choices = self.combobox_columns.get(col_key)
        if callable(choices):
            choices = choices()

        if choices is not None:
            editor = ttk.Combobox(self.tree, values=choices, state="readonly")
            editor.set(current_value)
        else:
            editor = ttk.Entry(self.tree)
            editor.insert(0, current_value)

        editor.place(
            x=x,
            y=y,
            width=width,
            height=height,
        )

        editor.focus_set()
        if hasattr(editor, "select_range"):
            try:
                editor.select_range(0, tk.END)
            except Exception:
                pass

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
        if choices is not None:
            editor.bind(
                "<<ComboboxSelected>>",
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
        
