import tkinter as tk
from tkinter import messagebox, ttk


class EditableTable(ttk.LabelFrame):
    """Editable table based on ttk.Treeview.

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
    ):
        super().__init__(parent, text=title, padding=8)

        self.columns = columns
        self.column_keys = [col[0] for col in columns]
        self.active_editor = None

        self.tree = ttk.Treeview(
            self,
            columns=self.column_keys,
            show="headings",
            height=height,
        )

        for key, heading, width in columns:
            self.tree.heading(key, text=heading)
            self.tree.column(key, width=width, anchor="center")

        self.tree.pack(fill="both", expand=True)

        buttons = ttk.Frame(self)
        buttons.pack(fill="x", pady=(6, 0))

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
            text="Double-click a cell to edit it directly.",
        ).pack(side="left", padx=(12, 0))

        self.tree.bind("<Double-1>", self._start_cell_edit)
        self.tree.bind("<Button-1>", self._handle_single_click)

    # ------------------------------------------------------------------
    # Public API used by app.py
    # ------------------------------------------------------------------
    def add_blank_row(self):
        empty_values = tuple("" for _ in self.column_keys)
        item_id = self.tree.insert("", "end", values=empty_values)

        self.tree.selection_set(item_id)
        self.tree.focus(item_id)

        # Open editor on first cell immediately.
        self.after(50, lambda: self._edit_cell(item_id, "#1"))

    def remove_selected(self):
        self._destroy_active_editor(save=False)

        selected = self.tree.selection()

        if not selected:
            messagebox.showinfo("No selection", "Select one or more rows to remove.")
            return

        for item in selected:
            self.tree.delete(item)

    def set_rows(self, rows: list[tuple]) -> None:
        self.clear()

        for row in rows:
            self.tree.insert("", "end", values=row)

    def get_rows(self) -> list[tuple]:
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
        # If user clicks elsewhere while editing, save current editor first.
        if self.active_editor is not None:
            self._destroy_active_editor(save=True)

    def _start_cell_edit(self, event):
        region = self.tree.identify("region", event.x, event.y)

        if region != "cell":
            return

        item_id = self.tree.identify_row(event.y)
        column_id = self.tree.identify_column(event.x)

        if not item_id or not column_id:
            return

        self._edit_cell(item_id, column_id)

    def _edit_cell(self, item_id: str, column_id: str):
        self._destroy_active_editor(save=True)

        bbox = self.tree.bbox(item_id, column_id)

        if not bbox:
            return

        x, y, width, height = bbox
        column_index = int(column_id.replace("#", "")) - 1

        current_values = list(self.tree.item(item_id, "values"))

        while len(current_values) < len(self.column_keys):
            current_values.append("")

        current_value = current_values[column_index]

        editor = tk.Entry(self.tree)
        editor.insert(0, current_value)
        editor.select_range(0, tk.END)
        editor.focus_set()

        editor.place(
            x=x,
            y=y,
            width=width,
            height=height,
        )

        self.active_editor = {
            "widget": editor,
            "item_id": item_id,
            "column_index": column_index,
            "original_value": current_value,
        }

        editor.bind("<Return>", lambda _event: self._destroy_active_editor(save=True))
        editor.bind("<KP_Enter>", lambda _event: self._destroy_active_editor(save=True))
        editor.bind("<Escape>", lambda _event: self._destroy_active_editor(save=False))
        editor.bind("<FocusOut>", lambda _event: self._destroy_active_editor(save=True))

    def _destroy_active_editor(self, save: bool):
        if self.active_editor is None:
            return

        editor = self.active_editor["widget"]
        item_id = self.active_editor["item_id"]
        column_index = self.active_editor["column_index"]
        original_value = self.active_editor["original_value"]

        if save and self.tree.exists(item_id):
            new_value = editor.get()

            values = list(self.tree.item(item_id, "values"))

            while len(values) < len(self.column_keys):
                values.append("")

            values[column_index] = new_value
            self.tree.item(item_id, values=values)

        elif not save and self.tree.exists(item_id):
            values = list(self.tree.item(item_id, "values"))

            while len(values) < len(self.column_keys):
                values.append("")

            values[column_index] = original_value
            self.tree.item(item_id, values=values)

        editor.destroy()
        self.active_editor = None
