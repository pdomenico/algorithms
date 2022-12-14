import math


class Heap:
    def __init__(self, is_min_heap: bool):
        self.is_min_heap = is_min_heap
        self.values = []

    def size(self) -> int:
        return len(self.values)

    def comp(self, a: int, b: int) -> bool:
        if self.is_min_heap:
            return a <= b
        return a >= b

    def peek(self) -> int:
        return self.values[0]

    def get_parent_index(self, child_index: int) -> int | None:
        if child_index == 0:
            return None
        return math.floor((child_index - 1) / 2)

    def get_left_child_index(self, parent_index: int) -> int | None:
        index = parent_index * 2 + 1
        if index > (self.size() - 1):
            return None
        return index

    def get_right_child_index(self, parent_index: int) -> int | None:
        index = parent_index * 2 + 2
        if index > (self.size() - 1):
            return None
        return index

    def heapify_up(self, from_index: int):
        parent_index = self.get_parent_index(from_index)
        if parent_index == None:
            return

        if self.comp(self.values[parent_index], self.values[from_index]):
            return

        self.values[parent_index], self.values[from_index] = (
            self.values[from_index],
            self.values[parent_index],
        )
        self.heapify_up(parent_index)

    def heapify_down(self, from_index: int):
        left_child_index = self.get_left_child_index(from_index)
        right_child_index = self.get_right_child_index(from_index)
        chosen_child_index = 0

        if left_child_index == None and right_child_index == None:
            return
        elif right_child_index == None:
            chosen_child_index = left_child_index
        elif left_child_index == None:
            chosen_child_index = right_child_index
        elif self.comp(self.values[right_child_index], self.values[left_child_index]):
            chosen_child_index = right_child_index
        else:
            chosen_child_index = left_child_index

        if chosen_child_index == None or self.comp(
            self.values[from_index], self.values[chosen_child_index]
        ):
            return

        self.values[chosen_child_index], self.values[from_index] = (
            self.values[from_index],
            self.values[chosen_child_index],
        )
        self.heapify_down(chosen_child_index)

    def pop_root(self) -> int:
        self.values[0], self.values[-1] = self.values[-1], self.values[0]
        res = self.values.pop()
        self.heapify_down(0)
        return res

    def add(self, new_element: int):
        self.values.append(new_element)
        self.heapify_up(self.size() - 1)

    def print(self):
        i = 0
        while True:
            for j in range(0, 2**i):
                index = 2**i + j - 1
                if index > self.size() - 1:
                    print("")
                    return
                if j == 2**i - 1:
                    print(self.values[index])
                else:
                    print(f"{self.values[index]} | ", end="")

            i += 1


max_heap = Heap(False)
min_heap = Heap(True)

median_sum = 0
with open("numbers.txt") as numbers:
    for (i, line) in enumerate(numbers):
        number = int(line)

        if min_heap.size() == 0 and max_heap.size() == 0:
            max_heap.add(number)
            median_sum += number
            continue

        if number >= max_heap.peek():
            min_heap.add(number)
        else:
            max_heap.add(number)

        if min_heap.size() > max_heap.size():
            max_heap.add(min_heap.pop_root())

        if max_heap.size() > min_heap.size() + 1:
            min_heap.add(max_heap.pop_root())

        median_sum += max_heap.peek()

print(median_sum % 10000)
