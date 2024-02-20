use std::ptr;

const N: usize = 303;


struct List<T> {
    head: Link<T>,
    length: usize,
    step_size: usize,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    value: T,
    next: Link<T>,
}


impl List<usize> {
    fn new(step_size: usize) -> Self {
        let mut list = List { head: ptr::null_mut(), length: 0, step_size };
        list.add_node(0);

        list
    }

    fn add_node(&mut self, value: usize) {
        if self.length > 0 {
            self.cycle(self.step_size % self.length);
        }

        unsafe {
            let next_node = Box::into_raw(Box::new(Node {
                value,
                next: ptr::null_mut(),
            }));

            if value == 0 {
                self.head = next_node;
                (*next_node).next = next_node;
            } else {
                (*next_node).next = (*self.head).next;
                (*self.head).next = next_node;
            }
        }
        self.length += 1;
        self.cycle(1);
    }

    fn cycle(&mut self, steps: usize) {
        for _ in 0..steps {
            self.move_next();
        }
    }

    fn move_next(&mut self) {
        unsafe {
            self.head = (*self.head).next;
        }
    }

    // fn cycle_to(&mut self, target: usize) {
    //     while self.get_current_value() != target {
    //         self.move_next()
    //     }
    // }

    fn get_current_value(&self) -> usize {
        unsafe { (*self.head).value }
    }

    // fn print_elements(&self) {
    //     let mut nums = Vec::new();
    //
    //
    //     unsafe {
    //         println!("Head: {}", (*self.head).value);
    //
    //         let mut curr = self.head;
    //         for _ in 0..self.length {
    //             let v = (*curr).value;
    //             nums.push(v);
    //
    //             curr = (*curr).next
    //         }
    //
    //         println!("Head: {}", (*self.head).value);
    //     }
    //
    //     println!("{}", nums.iter().map(|x| x.to_string()).join(" -> "));
    // }
}


pub fn solve_a() {
    let mut list = List::new(N);

    for i in 1..=2017 {
        list.add_node(i);
    }

    list.cycle(1);
    let ans = list.get_current_value();

    assert_eq!(ans, 1971);
    println!("Solution A: {}", ans);
}


pub fn solve_b() {
    let mut ans = 0;
    let mut i = 0;  // index position

    for t in 1..=50_000_000 {
        // this checks the position of the next item
        i = (i + N) % t + 1;
        // if the position is the first index in the list, record it
        if i == 1 {
            ans = t;
        }
    }

    println!("Solution B: {}", ans)
}