use core::str::FromStr;

fn run_impl<'a>(input: &'a str, is_part1: bool) -> usize {
    let mut mapping = [""; 100];
    let mut mapping_size = 0;
    let mut map = move |name: &'a str| -> usize {
        if let Some(i) = mapping[..mapping_size].iter().position(|&x| x == name) {
            i
        } else {
            mapping[mapping_size] = name;
            mapping_size += 1;
            mapping_size - 1
        }
    };
    let start = map("AA");

    let mut valve_states = [None; 100];
    let mut flows = [0; 100];
    let mut state_bits = 0;

    let mut edges = [0; 1000];
    let mut edges_size = 0;
    let mut edge_ranges = [(0, 0); 100];

    for line in input.lines() {
        let (rate, edges_str) = line
            .split_once("; tunnels lead to valves ")
            .or_else(|| line.split_once("; tunnel leads to valves "))
            .or_else(|| line.split_once("; tunnels leads to valve "))
            .or_else(|| line.split_once("; tunnel leads to valve "))
            .unwrap();
        let (valve, rate) = rate.split_once(" has flow rate=").unwrap();
        let valve = map(valve.strip_prefix("Valve ").unwrap());
        let rate = i16::from_str(rate).unwrap();
        if rate > 0 {
            valve_states[valve] = Some(state_bits);
            flows[state_bits] = rate;
            state_bits += 1;
        }
        let start_edges = edges_size;
        for edge in edges_str.split(", ") {
            edges[edges_size] = map(edge);
            edges_size += 1;
        }
        edge_ranges[valve] = (start_edges, edges_size);
    }

    let index = |state: u32, pos: usize, other_pos: usize, time: u32| -> u32 {
        ((time * 60 + pos as u32) * 60 + other_pos as u32) * 2_u32.pow(15) + state
    };
    let from_index = |mut index: u32| {
        let state = index % 2_u32.pow(15);
        index /= 2_u32.pow(15);
        let other_pos = index % 60;
        index /= 60;
        let pos = index % 60;
        index /= 60;
        let time = index;
        (state, pos as usize, other_pos as usize, time)
    };

    assert!(state_bits <= 15);
    assert!(mapping_size <= 60);
    let mut q = [index(0_u32, start, start, 0); 2_usize.pow(15) * 60 * 60];
    let mut q_pos = 0;
    let mut q_size = 1;
    let mut visited = [-1_i16; 2_usize.pow(15) * 60 * 60 * 2];
    visited[(q[0] as usize) % visited.len()] = 0;
    let mut best = 0;
    let end_time = if is_part1 { 30 } else { 26 };
    while q_pos != q_size {
        let (state, place, other_place, time) = from_index(q[q_pos]);
        let flow = visited[(q[q_pos] as usize) % visited.len()];
        q_pos = (q_pos + 1) % q.len();

        let mut next_flow = flow;
        for (i, rate) in flows[..state_bits].iter().enumerate() {
            if state & (1 << i) != 0 {
                next_flow += rate;
            }
        }

        let mut add = |next_state, place: usize, other_place| {
            if time + 1 >= end_time {
                best = best.max(next_flow);
                return;
            }
            let next_index = index(
                next_state,
                place.max(other_place),
                other_place.min(place),
                time + 1,
            );
            if visited[(next_index as usize) % visited.len()] < next_flow {
                q[q_size] = next_index;
                visited[(next_index as usize) % visited.len()] = next_flow;
                q_size = (q_size + 1) % q.len();
                assert!(q_size != q_pos);
            }
        };

        let mut place_turned_on = state;
        if let Some(bit) = valve_states[place] {
            place_turned_on = state | (1 << bit);
        }
        let mut other_place_turned_on = state;
        if !is_part1 {
            if let Some(bit) = valve_states[other_place] {
                other_place_turned_on = state | (1 << bit);
            }
            for &other_target in &edges[edge_ranges[other_place].0..edge_ranges[other_place].1] {
                add(place_turned_on, place, other_target);
            }
        }
        add(place_turned_on | other_place_turned_on, place, other_place);
        for &target in &edges[edge_ranges[place].0..edge_ranges[place].1] {
            add(other_place_turned_on, target, other_place);

            if !is_part1 {
                for &other_target in &edges[edge_ranges[other_place].0..edge_ranges[other_place].1]
                {
                    add(state, target, other_target);
                }
            }
        }
    }

    best.try_into().unwrap()
}

pub fn run(input: &str) -> (usize, usize) {
    (run_impl(input, true), run_impl(input, false))
}
