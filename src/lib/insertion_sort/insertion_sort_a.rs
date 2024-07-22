// in [5,2,4,6,1,3]
// out[1,2,3,4,5,6]

use std::fmt::Debug;

pub struct InsertionSort<T:>{
    list:Vec<T>
}

impl<T:PartialEq+PartialOrd+Copy+Debug> InsertionSort<T>{
    pub fn sort(&mut self){
        let A:&mut Vec<T>  =&mut self.list;
        println!("{A:?}");
        for j in 2..=A.len(){
            let key= A[j-1];
            let mut i=j -2;
            let mut i_zero:bool=false;
            while A[i]>key {
                A[i+1]=A[i];
                if i==0{
                    println!("\t{A:?},1-{key:?}");
                    A[0]=key;
                    i_zero=true;
                    break;
                }else {
                    println!("\t{A:?},{}-{key:?}",i+1);
                    i-=1;
                }
            }
            if !i_zero{
                A[i+1]=key;
            }
            println!("{A:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InsertionSort;

    #[test]
    fn insertion_sort_a() {
        let mut insert_sort:InsertionSort<u32>=InsertionSort{
            list:vec![5,2,4,6,1,3]
        };
        insert_sort.sort();
        assert_eq!(vec![1, 2, 3, 4, 5, 6],insert_sort.list);

        println!(" ");

        let mut insert_sort:InsertionSort<u32>=InsertionSort{
            list:vec![1,7,5,3,0,6,3,5,8,7,5,9]
        };
        insert_sort.sort();
        assert_eq!(vec![0, 1, 3, 3, 5, 5, 5, 6, 7, 7, 8, 9]
                   ,insert_sort.list);


        println!(" ");

        let mut insert_sort:InsertionSort<i32>=InsertionSort{
            list:vec![1,7,5,3,0,6,3,5,8,7,5,9,-2,-7,-1]
        };
        insert_sort.sort();
        assert_eq!(vec![-7, -2, -1, 0, 1, 3, 3, 5, 5, 5, 6, 7, 7, 8, 9]
                   ,insert_sort.list);


    }



}