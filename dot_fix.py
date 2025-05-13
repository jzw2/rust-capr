
import re
import sys
from collections import defaultdict

def merge_dot_edges(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    header_lines = []
    footer_lines = []
    node_lines = []
    edge_dict = defaultdict(list)
    edge_attrs = {}

    in_graph = False

    for line in lines:
        stripped = line.strip()

        if stripped.startswith('digraph'):
            header_lines.append(line)
            in_graph = True
        elif in_graph and stripped == '}':
            footer_lines.append(line)
            in_graph = False
        elif in_graph:
            edge_match = re.match(r'(\d+)\s*->\s*(\d+)\s*\[label\s*=\s*"([^"]+)"(.*?)\];', stripped)
            if edge_match:
                src, dst, label, rest = edge_match.groups()
                edge_dict[(src, dst)].append(label)
                if (src, dst) not in edge_attrs:
                    edge_attrs[(src, dst)] = rest.strip()
            else:
                node_lines.append(line)
        else:
            header_lines.append(line)

    with open(filename, 'w') as f:
        # Write header
        for line in header_lines:
            f.write(line)

        # Write nodes
        for line in node_lines:
            f.write(line)

        # Write merged edges
        for (src, dst), labels in edge_dict.items():
            merged_label = ", ".join(labels)
            rest = edge_attrs.get((src, dst), '')
            rest_part = f" {rest}" if rest else ''
            f.write(f'    {src} -> {dst} [label = "{merged_label}"{rest_part}];\n')

        # Write footer
        for line in footer_lines:
            f.write(line)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python merge_edges.py <dot_file>")
        sys.exit(1)
    merge_dot_edges(sys.argv[1])
