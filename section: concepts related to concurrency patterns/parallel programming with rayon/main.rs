fn merge_sort_par(arr: &mut [i32]) {
  if arr.len() <= 1 {
      return;
  }

  let mid = arr.len() / 2;
  let (left, right) = arr.split_at_mut(mid);

  rayon::join(|| merge_sort_par(left), || merge_sort_par(right));

  let mut i = 0;
    let mut j = mid;
    let mut temp = Vec::with_capacity(arr.len());

    while i < mid && j < arr.len() {
        if arr[i] < arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }

    while i < mid {
        temp.push(arr[i]);
        i += 1;
    }

    while j < arr.len() {
        temp.push(arr[j]);
        j += 1;
    }

    arr.copy_from_slice(&temp);
}

fn main() {
  let mut arr = vec![8, 2, 5, 9, 1, 3, 7, 6, 4];
  merge_sort_par(&mut arr);
  println!("{:?}", arr);
}
