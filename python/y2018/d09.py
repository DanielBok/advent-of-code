# n = 478
# m = 71240
#
# scores = {i: 0 for i in range(n)}
#
# data = [0, 1]
# curr_index = 1
# for i in range(2, m):
#     if i % 23 == 0:
#         curr_index -= 7
#         if curr_index < 0:
#             curr_index += 1
#         value = data.pop(curr_index)
#         scores[i % n] += i + value
#
#     else:
#         curr_index = ((curr_index + 1) % len(data)) + 1
#         data.insert(curr_index, i)
#
# print(max(scores.values()))


class Node:
    def __init__(self, value: int):
        self.value = value
        self.left: Node = None
        self.right: Node = None

    def set_neighbour(self, left: 'Node', right: 'Node'):
        self.left = left
        self.right = right

    def insert_right(self, node: 'Node'):
        self.right.left = node
        node.right = self.right

        self.right = node
        node.left = self

    def remove(self) -> 'Node':
        self.left.right = self.right
        self.right.left = self.left

        return self.right

    def next(self, d=1) -> 'Node':
        node = self
        for i in range(d):
            node = node.right
        return node

    def prev(self, d=1) -> 'Node':
        node = self
        for i in range(d):
            node = node.left
        return node


class Game:
    def __init__(self, players: int, marbles: int):

        node0 = Node(0)
        node1 = Node(1)
        node0.set_neighbour(node1, node1)
        node1.set_neighbour(node0, node0)

        self.curr: Node = node1
        self._p = players
        self._m = marbles

    def get_score(self, v):
        self.curr = self.curr.prev(7)
        v += self.curr.value
        self.curr = self.curr.remove()
        return v

    def make_node(self, value):
        node = Node(value)
        self.curr = self.curr.next()
        self.curr.insert_right(node)
        self.curr = self.curr.next()

    def run(self):
        scores = {i: 0 for i in range(self._p)}
        for i in range(2, self._m):
            if i % 23 == 0:
                scores[i % self._p] += self.get_score(i)
            else:
                self.make_node(i)

        return max(scores.values())


Game(9, 39).run()
for n, m in [(10, 1618), (13, 7999), (17, 1104), (21, 6111), (30, 5807)]:
    print(Game(n, m).run())

