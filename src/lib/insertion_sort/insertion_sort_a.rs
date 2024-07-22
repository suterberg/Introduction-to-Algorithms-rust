// in [5,2,4,6,1,3]
// out[1,2,3,4,5,6]

pub struct InsertionSort<T>{
    list:Vec<T>
}

impl InsertionSort<u32>{
    pub fn sort(&mut self){
        let A =&mut self.list;
        println!("{A:?}");
        for j in 1..A.len(){
            let key= A[j];
            let mut i:isize=j as isize-1;
            while i>=0 && A[i as usize]>key {
                A[(i+1)as usize]=A[i as usize];
                i-=1;
                println!("\t{A:?},{i}");
            }
            A[(i+1)as usize]=key;
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

    }



}