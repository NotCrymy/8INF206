import rust_core
import argparse
import os
import random

def compute_layout(result):
    if result is None:
        print("Error computing layout")
        exit(1)
    else:
        print("Layout computed successfully")
        print(f"{result.nodes_positions}")
        print(f"{result.edges_routes}")

def orientation(p, q, r):
    val = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    eps = 1e-6

    if val > eps:
        return 1
    elif val < -eps:
        return 2
    else:
        return 0


def segments_intersect(p1, q1, p2, q2):
    o1 = orientation(p1, q1, p2)
    o2 = orientation(p1, q1, q2)
    o3 = orientation(p2, q2, p1)
    o4 = orientation(p2, q2, q1)

    # ignorer colinéaire
    if o1 == 0 or o2 == 0 or o3 == 0 or o4 == 0:
        return False

    return o1 != o2 and o3 != o4


def evaluate_layout(layout):
    bends = 0
    crossings = 0

    # ---- bends ----
    for _, points, _ in layout.edges_routes:
        if len(points) >= 3:
            for i in range(len(points) - 2):
                (x1, y1) = points[i]
                (x2, y2) = points[i + 1]
                (x3, y3) = points[i + 2]

                if not ((x1 == x2 == x3) or (y1 == y2 == y3)):
                    bends += 1

    # ---- crossings ----
    edges = [points for _, points, _ in layout.edges_routes]

    for i in range(len(edges)):
        for j in range(i + 1, len(edges)):
            e1 = edges[i]
            e2 = edges[j]

            p1, q1 = e1[0], e1[-1]
            p2, q2 = e2[0], e2[-1]

            if p1 == p2 or p1 == q2 or q1 == p2 or q1 == q2:
                continue

            if segments_intersect(p1, q1, p2, q2):
                crossings += 1

    return {
        "bends": bends,
        "crossings": crossings
    }

def export_svg(layout, filepath, filename):
    import random

    node_width = 80
    node_height = 40
    padding = 50

    # bounding box for nodes
    min_x = min(x for _, _, x, _, _ in layout.nodes_positions)
    min_y = min(y for _, _, _, y, _ in layout.nodes_positions)
    max_x = max(x for _, _, x, _, _ in layout.nodes_positions)
    max_y = max(y for _, _, _, y, _ in layout.nodes_positions)

    offset_x = padding - min_x
    offset_y = padding - min_y

    width = (max_x - min_x) + node_width + 2 * padding
    height = (max_y - min_y) + node_height + 2 * padding

    if not os.path.exists(filepath):
        os.makedirs(filepath)

    full_path = os.path.join(filepath, filename)

    # generate a random color for each source node to color edges consistently
    color_map = {}

    def get_color(key):
        if key not in color_map:
            color_map[key] = (
                random.randint(50, 255),
                random.randint(50, 255),
                random.randint(50, 255),
            )
        return color_map[key]

    with open(full_path, "w") as f:
        f.write(f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">\n')

        # edges
        for source_id, edge, crossing in layout.edges_routes:
            r, g, b = get_color(source_id)

            points = " ".join(f"{x + offset_x},{y + offset_y}" for x, y in edge)

            f.write(
                f'<polyline points="{points}" '
                f'style="stroke:rgb({r},{g},{b});stroke-width:2" fill="none"/>\n'
            )

        # nodes
        for node_id, node_string, x, y, is_dummy in layout.nodes_positions:
            if is_dummy:
                continue

            x += offset_x
            y += offset_y

            rect_x = x - node_width / 2
            rect_y = y - node_height / 2

            f.write(
                f'<rect x="{rect_x}" y="{rect_y}" width="{node_width}" height="{node_height}" fill="white" stroke="black"/>'
                f'<text x="{rect_x + 5}" y="{rect_y + node_height / 2}" fill="black" font-size="10">{node_string}</text>\n'
            )

        f.write("</svg>")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--file", type=str, default="graphs/graph.json", help="Path to the input graph JSON file (default: graphs/graph.json)")
    parser.add_argument("--path", type=str, default="results", help="Path to save the SVG file (default: results/)")
    parser.add_argument("--name", type=str, default="graph.svg", help="Name of the SVG file (default: graph.svg)")
    parser.add_argument("--evaluate", action="store_true", help="Evaluate the layout and print the score")

    args = parser.parse_args()

    if args.name[-4:].lower() != ".svg":
        args.name += ".svg"
    
    if not os.path.exists(args.path):
        os.makedirs(args.path)

    result = rust_core.compute_layout_dto(args.file)

    compute_layout(result)
    export_svg(result, filepath=args.path, filename=args.name)
    print(f"SVG exported successfully to {args.path}/{args.name}")

    if args.evaluate:
        evaluation = evaluate_layout(result)
        print(f"\nLayout evaluation: {evaluation}")