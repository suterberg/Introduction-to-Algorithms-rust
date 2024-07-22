use std::fmt::Debug;


pub struct Merge<T>{
    list:Vec<T>
}

impl<T:PartialEq+PartialOrd+Debug+Copy> Merge<T>{
    pub fn merge(&mut self,p:usize,q:usize,r:usize){
        let A:&mut Vec<T>=&mut self.list;
        println!("{p},{q},{r}");
        let L=A[p..=q].to_vec();
        let R=A[q+1..=r].to_vec();
        let len_r=R.len();
        let len_l=L.len();
        let mut i:usize=0;
        let mut j:usize=0;
        for k in p..=r{
            if j==len_r||(i<len_l&& L[i]<=R[j]) {
                A[k]=L[i];
                i=i+1;
            }else {
                A[k]=R[j];
                j+=1;
            }
        }
        println!("{A:?}");

    }

    pub fn merge_sort(&mut self,p:usize,r:usize){
        if p<r{
            let q=(p+r)/2;
            println!("sort{p},{q}");
            self.merge_sort(p,q);
            println!("sort{},{r}",q+1);
            self.merge_sort(q+1,r);
            println!("merge{p} {q} {r}");
            self.merge(p,q,r)
        }
    }
    pub fn sort(&mut self){
        self.merge_sort(0,self.list.len()-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Merge;

    #[test]
    fn merge_sort() {
        let mut insert_sort:Merge<u32>=Merge{
            list:vec![5,2,4,6,1,3]
        };
        insert_sort.sort();
        assert_eq!(vec![1, 2, 3, 4, 5, 6],insert_sort.list);

        println!(" ");

        let mut insert_sort:Merge<u32>=Merge{
            list:vec![1,7,5,3,0,6,3,5,8,7,5]
        };
        insert_sort.sort();
        assert_eq!(vec![0, 1, 3, 3, 5, 5, 5, 6, 7, 7, 8 ]
                   ,insert_sort.list);


        println!(" ");

        let mut insert_sort:Merge<i32>=Merge{
            list:vec![1,7,5,3,0,6,3,5,8,7,5,9,-2,-7,-1]
        };
        insert_sort.sort();
        assert_eq!(vec![-7, -2, -1, 0, 1, 3, 3, 5, 5, 5, 6, 7, 7, 8, 9]
                   ,insert_sort.list);


    }



}