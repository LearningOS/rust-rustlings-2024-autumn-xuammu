/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn quick_sort<T>(array: &mut [T],mut left:usize,mut right:usize)->usize
where
T:Ord+PartialOrd
{
    let flag = left;
    while left < right{
        while left<right && array[left] < array[flag] {
            left += 1;
        }
        while left < right && array[right] >= array[flag] {
            right -=1;
        }
        if left != right{
            array.swap(left, right);
        }
    }
    array.swap(flag, left);
    left
}
fn sort<T>(array: &mut [T])
where 
T:Ord+PartialOrd
{
    if array.len() <=1 {
        return;
    }
	let mut flag = quick_sort(array, 0, array.len()-1);
    if flag != 0{
        sort(& mut array[0..flag]);
    }
    if flag+1 < array.len(){
        sort(&mut array[flag+1..]);
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}