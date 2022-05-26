
pub fn run()
{
    // Primitive types, its ok.
    let mut arr1 = [1,2,3];
    let arr2 = arr1; // arr1 is copied into arr2. Modifying arr1 later doesnt change arr2
    arr1[0] = 0;

    println!("{:?}", (arr1, arr2));

    let mut v1 = vec![1,2,3];
    // let v2 = &v1; // Has to be a reference if not primitive value
    let v2 = v1.clone(); // Can also clone value when implicit copy is not available. Can modify v1, with reference we cant.
    v1[0] = 0;

    println!("{:?}", (v1, v2)); // if v2 = &v1, v1 has to be &v1 here
}