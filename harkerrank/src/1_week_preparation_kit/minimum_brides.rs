use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

// q = [1, 2, 3, 5, 4, 6, 7, 8]
// Print '1'

// q = [4, 1, 2, 3]
// Print 'Too chaotic'

// q = [1, 2, 5, 3, 7, 8, 6, 4]
// Print '7'
// fn minimumBribes(q: &[i32]) {
//   let mut bribes = 0;
//   let mut chaotic = false;

//   for i in 0..q.len() {
//     let bribe = q[i] - (i as i32 + 1);

//     if bribe > 2 {
//       chaotic = true;
//       break;
//     }

//     for j in 0..i {
//       if q[j] > q[i] {
//         bribes += 1;
//       }
//     }
//   }

//   if chaotic {
//     println!("Too chaotic");
//   } else {
//     println!("{}", bribes);
//   }
// }

// KO: 뒤에서 부터 접근
// EN: Approach from the back
fn minimumBribes(q: &[i32]) {
  let mut q = q.to_vec();
  let mut bribes = 0;

  for i in (0..q.len()).rev() {
    let expected_value = (i + 1) as i32;

    if q[i] != expected_value {
      if i >= 2 && q[i - 2] == expected_value {
        bribes += 2;
        q.swap(i - 2, i - 1);
        q.swap(i - 1, i);
      } else if i >= 1 && q[i - 1] == expected_value {
        bribes += 1;
        q.swap(i - 1, i);
      } else {
        println!("Too chaotic");
        return;
      }
    }
  }

  println!("{}", bribes);
}

fn main() {
  let stdin = io::stdin();
  let mut stdin_iterator = stdin.lock().lines();

  let t = stdin_iterator
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .parse::<i32>()
    .unwrap();

  for _ in 0..t {
    let n = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .trim()
      .parse::<i32>()
      .unwrap();

    let q: Vec<i32> = stdin_iterator
      .next()
      .unwrap()
      .unwrap()
      .trim_end()
      .split(' ')
      .map(|s| s.to_string().parse::<i32>().unwrap())
      .collect();

    minimumBribes(&q);
  }
}
