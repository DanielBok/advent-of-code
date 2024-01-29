use std::iter::Iterator;

fn get_input() -> Vec<i32> {
    "51585
137484
73634
71535
87274
74243
127025
66829
138729
145459
118813
82326
82518
145032
148699
105958
103969
72689
145061
70385
53104
107851
103392
107051
123475
123918
56709
89284
86208
71943
109257
108272
124811
142709
115650
53607
142891
144135
114277
138671
111998
70838
69802
107210
103319
60377
58639
131863
100807
118360
52573
108207
128009
96180
148492
112914
72867
140991
131267
125123
58393
129615
87239
63085
59231
95007
147712
109838
89829
55634
96163
52323
106701
141511
125349
137267
50694
53692
57466
117769
63535
101708
113593
79163
112327
91994
129674
58076
145062
122730
102481
109994
136271
111178
117920
107933
104305
99613
68482
126543"
        .split('\n')
        .map(|e| {
            e.parse::<i32>().unwrap_or_else(|_| {
                panic!("Could not parse {} as an integer", e)
            })
        })
        .collect::<Vec<i32>>()
}


fn required_fuel(mass: i32) -> i32 {
    if mass < 9 { 0 } else { mass / 3 - 2 }
}

pub fn solve_a() {
    let inp = get_input();

    let ans = inp.iter().map(|&x| required_fuel(x)).sum::<i32>();

    assert_eq!(ans, 3384232);
    println!("Solution A: {}", ans)
}

pub fn solve_b() {
    let inp = get_input();

    let ans = inp.iter().map(|& x| {
        let mut x = x;
        let mut total = 0;
        while x > 0 {
            x = required_fuel(x);
            total += x;
        }

        total
    }).sum::<i32>();

    assert_eq!(ans, 5073456);
    println!("Solution B: {}", ans);
}

