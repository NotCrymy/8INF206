import json
import customtkinter as ctk
from tkinter import messagebox

ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("blue")

nodes = []
edges = []

def refresh_dropdowns():
    src_menu.configure(values=nodes)
    dst_menu.configure(values=nodes)

def refresh_ui():
    node_listbox.delete("1.0", "end")
    edge_listbox.delete("1.0", "end")

    for i, n in enumerate(nodes):
        node_listbox.insert("end", f"{i}: {n}\n")

    for src, dst in edges:
        edge_listbox.insert(
            "end",
            f"{nodes[src]} ({src}) → {nodes[dst]} ({dst})\n"
        )

    refresh_dropdowns()

def add_node():
    name = node_entry.get()
    if name:
        nodes.append(name)
        node_listbox.insert("end", f"{len(nodes)-1}: {name}\n")
        node_entry.delete(0, "end")
        refresh_dropdowns()

def delete_node():
    try:
        if not node_listbox.tag_ranges("sel"):
            messagebox.showwarning("Attention", "Sélectionne un node")
            return

        selection = node_listbox.get("sel.first", "sel.last").strip()

        idx = int(selection.split(":")[0])

        nodes.pop(idx)

        new_edges = []
        for src, dst in edges:
            if src == idx or dst == idx:
                continue
            new_src = src - 1 if src > idx else src
            new_dst = dst - 1 if dst > idx else dst
            new_edges.append([new_src, new_dst])

        edges.clear()
        edges.extend(new_edges)

        refresh_ui()

    except Exception as e:
        messagebox.showerror("Erreur", str(e))

def add_edge():
    src = src_var.get()
    dst = dst_var.get()

    if src not in nodes or dst not in nodes:
        messagebox.showerror("Erreur", "Node invalide")
        return

    src_idx = nodes.index(src)
    dst_idx = nodes.index(dst)

    edges.append([src_idx, dst_idx])
    edge_listbox.insert("end", f"{src} ({src_idx}) → {dst} ({dst_idx})\n")

def delete_edge():
    try:
        if not edge_listbox.tag_ranges("sel"):
            messagebox.showwarning("Attention", "Sélectionne un edge")
            return

        selection = edge_listbox.get("sel.first", "sel.last").strip()

        parts = selection.split("→")
        src_idx = int(parts[0].split("(")[1].split(")")[0])
        dst_idx = int(parts[1].split("(")[1].split(")")[0])

        edges.remove([src_idx, dst_idx])

        refresh_ui()

    except Exception as e:
        messagebox.showerror("Erreur", str(e))

def save_json():
    data = {
        "nodes": nodes,
        "edges": edges
    }

    with open("graphs/custom_graph.json", "w") as f:
        json.dump(data, f, indent=2)

    messagebox.showinfo("Succès", "custom_graph.json généré !")

# UI
root = ctk.CTk()
root.title("Graph Builder")
root.geometry("700x700")

# --- Nodes ---
ctk.CTkLabel(root, text="Ajouter un node").pack(pady=5)

node_entry = ctk.CTkEntry(root)
node_entry.pack(pady=5)

ctk.CTkButton(root, text="Ajouter node", command=add_node).pack(pady=5)
ctk.CTkButton(root, text="Supprimer node", command=delete_node).pack(pady=5)

node_listbox = ctk.CTkTextbox(root, height=120)
node_listbox.pack(pady=10)

# --- Edges ---
ctk.CTkLabel(root, text="Ajouter un edge").pack(pady=5)

src_var = ctk.StringVar()
dst_var = ctk.StringVar()

src_menu = ctk.CTkOptionMenu(root, variable=src_var, values=nodes)
src_menu.pack(pady=5)

dst_menu = ctk.CTkOptionMenu(root, variable=dst_var, values=nodes)
dst_menu.pack(pady=5)

ctk.CTkButton(root, text="Ajouter edge", command=add_edge).pack(pady=5)
ctk.CTkButton(root, text="Supprimer edge", command=delete_edge).pack(pady=5)

edge_listbox = ctk.CTkTextbox(root, height=120)
edge_listbox.pack(pady=10)

# --- Save ---
ctk.CTkButton(root, text="Générer JSON", command=save_json).pack(pady=20)

root.mainloop()