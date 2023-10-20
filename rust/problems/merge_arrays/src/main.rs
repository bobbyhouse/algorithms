use std::ops::Range;
fn main() {
    let m: usize = 3;
    let n: usize = 3;
    let mut nums1 = vec![2, 4, 8, 0, 0, 0];
    let nums2 = vec![2, 3, 6];

    //let m: usize = 1;
    //let n: usize = 1;
    //let mut nums1 = vec![1, 0];
    //let nums2 = vec![2];

    //let m: usize = 2;
    //let n: usize = 2;
    //let mut nums1 = vec![1, 5, 0, 0];
    //let nums2 = vec![2, 5];

    //let m: usize = 2;
    //let n: usize = 2;
    //let mut nums1 = vec![1, 4, 0, 0];
    //let nums2 = vec![2, 5];

    let range1 : Range<usize> = 0..m;
    let mut range1 = range1.rev();

    let range2: Range<usize> = 0..n;
    let mut range2 = range2.rev();

    let range3: Range<usize> = 0..m+n;
    let mut range3 = range3.rev();

    let mut option_i = range1.next();
    let mut option_j = range2.next();
    let mut option_k = range3.next();

    loop {
        if option_i == None || option_j == None {
            break;
        }

        let i = option_i.unwrap();
        let j = option_j.unwrap();
        let k = option_k.unwrap();

        if nums1[i] > nums2[j] {
            nums1[k] = nums1[i];
            option_i = range1.next();
        } else {
            nums1[k] = nums2[j];
            option_j = range2.next();
        }
        option_k = range3.next();
    }

    //for j in range2 {
    //    option_k = range3.next();
    //    let k = option_k.unwrap();
    //    nums1[k] = nums2[j];
    //}

    println!("{:?}", nums1);

}



// mpl Solution {
//     pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

//         let range1 = (0..(m as usize));
//         let mut range1 = range1.rev();

//         let range2 = (0..(n as usize));
//         let mut range2 = range2.rev();

//         let range3 = (0..(m as usize + n as usize));
//         let mut range3 = range3.rev();

//         let mut option_i = range1.next();
//         let mut option_j = range2.next();
//         let mut option_k = range3.next();

//         loop {
//             if option_i == None || option_j == None {
//                 break;
//             }

//             let i = option_i.unwrap();
//             let j = option_j.unwrap();
//             let k = option_k.unwrap();

//             if nums1[i] > nums2[j] {
//                 nums1[k] = nums1[i];
//                 option_i = range1.next();
//             } else {
//                 nums1[k] = nums2[j];
//                 option_j = range2.next();
//             }

//             option_k = range3.next();
//         }

//         loop {
//             if option_j == None {
//                 break;
//             }

//             let j = option_j.unwrap();
//             let k = option_k.unwrap();
//             nums1[k] = nums2[j];

//             option_j = range2.next();
//             option_k = range3.next();
//         }
//     }
// }