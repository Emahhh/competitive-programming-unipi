/**
 * an implementation of binary search using recursion
 */
fn binary_search<T: Ord>(arr: &[T], needle: T, starting_index: usize) -> Option<usize> {
    // in order to use recursion, we need to pass the index (otherwise we would lose the information about the true index)

    let len: usize = arr.len();
    if(len == 0){ return None; }
    if(len == 1 && arr[0] == needle) { return Some(starting_index); }
    if (len == 1) {return None; }

    let middle: usize = len/2;
    if (arr[middle] == needle) {return Some(starting_index + middle); } // we need to return the true index (not just relative to this iteration)

    // let's search inthe first or second half
    if (needle < arr[middle]){ // the key could be on the left half
        return binary_search(&arr[0 .. middle], needle, starting_index);
    }
    // else, the key could be on the right half
    return binary_search(&arr[middle .. len], needle, starting_index+middle);
}


#[test]
pub fn myTests(){
    let prova1 = [1,2,3,99,101,150,1097,2000];
    let res: Option<usize> = binary_search(&prova1, 3456977, 0);
    println!("il valore è presente all'indice: {:?}", res);
    assert_eq!(res, None);

    let res: Option<usize> = binary_search(&prova1, 1, 0);
    println!("il valore è presente all'indice: {:?}", res);
    assert_eq!(res, Some(0));
}




pub fn main() {
    println!("Hello, reursive binary search!");
}

