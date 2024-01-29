from typing import List


def read_data():
    with open('d08.txt') as f:
        data = [int(i) for i in f.read().strip().split()]

    return data


def is_empty(arr):
    return len(arr) == 0


class Node:
    def __init__(self, *meta: int):
        self.meta = meta
        self.children: List[Node] = []

    def set_meta_data(self, *meta: int):
        self.meta = meta

    def add_child(self, child: 'Node'):
        self.children.append(child)

    def sum(self):
        return sum(self.meta) + sum(c.sum() for c in self.children)

    def ref_sum(self):
        num_child = len(self.children)
        if num_child == 0:
            return sum(self.meta)

        meta = sorted([i - 1 for i in self.meta if i - 1 < num_child])
        if len(meta) == 0:
            return 0

        return sum(self.children[i].ref_sum() for i in meta)


def form_tree():
    data = read_data()
    # data = [int(i) for i in '2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2'.split()]
    node_stack: List[Node] = []
    child_stack: List[int] = []
    meta_stack: List[int] = []
    c = 0

    while c < len(data):
        if is_empty(child_stack) or child_stack[-1] > 0:
            n = data[c]
            m = data[c + 1]
            c += 2

            if n == 0:
                node = Node(*data[c:c + m])
                c += m
                node_stack[-1].add_child(node)
                child_stack[-1] -= 1

            else:
                node = Node()
                node_stack.append(node)
                child_stack.append(n)
                meta_stack.append(m)

        else:
            child_stack.pop()
            node = node_stack.pop()
            m = meta_stack.pop()
            node.set_meta_data(*data[c:c + m])
            c += m

            if len(node_stack) > 0:
                node_stack[-1].add_child(node)
                child_stack[-1] -= 1
            else:
                return node


def q1():
    return form_tree().sum()


def q2():
    return form_tree().ref_sum()
