// & means borrowig 
fn use_slices(slice: &mut[i32]){
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

}

fn slices(){
    let mut data: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", data);
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}


fn main(){
    slices();
}