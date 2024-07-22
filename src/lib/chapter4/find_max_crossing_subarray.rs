use std::fmt::Debug;
use std::ops::Add;

pub struct FindSubarry<T>{
    list:Vec<T>
}
impl<T:PartialEq+PartialOrd+Debug+Copy+Add<Output = T>+From<u8>> FindSubarry<T>{

    pub fn find_max_crossing_subarray(&mut self,low:usize,mid:usize,high:usize)
        ->(usize,usize,T){
        let A:&mut Vec<T>=&mut self.list;
        let mut max_left:Option<usize>=None;
        let mut max_right:Option<usize>=None;
        let mut l_sum:Option<T>=None;
        let mut sum:T=0_u8.into();
        for i in (low..=mid).rev(){
            sum=A[i]+sum;
            if let None=l_sum{
                l_sum=Some(sum)
            }
            if Some(sum)>l_sum{
                l_sum=Some(sum);
                max_left=Some(i);
            }
        }
        let mut r_sum:Option<T>=None;
        let mut sum:T=0_u8.into();

        for j in (mid+1..=high){
            sum=sum+A[j];
            if let None=r_sum{
                r_sum=Some(sum)
            }
            if Some(sum)>r_sum{
                r_sum=Some(sum);
                max_right=Some(j);
            }
        }
        let mut sum:T=r_sum.unwrap_or(0.into())+l_sum.unwrap_or(0.into());
        return (max_left.unwrap_or(mid),max_right.unwrap_or(mid+1),sum)
    }


    pub fn find_maximum_subarray(&mut self,low:usize,high:usize)
    ->(usize,usize,T){
        let A:&mut Vec<T>=&mut self.list;
        if high==low{
            return (low,high,A[low])
        }else {
            let mid=(low+high)/2;
            let (l_low,l_high,l_sum)=self.find_maximum_subarray(low,mid);
            let (r_low,r_high,r_sum)=self.find_maximum_subarray(mid+1,high);
            let(c_low,c_high,c_sum)=self.find_max_crossing_subarray(low,mid,high);

            if l_sum>=r_sum && l_sum>=c_sum{
                return (l_low,l_high,l_sum)
            }else if r_sum>=l_sum&&r_sum>=c_sum {
                return (r_low,r_high,r_sum)
            }else {
                return (c_low,c_high,c_sum)
            }
        }
    }

    pub fn finds(&mut self)->(usize,usize,T){
        self.find_maximum_subarray(0,self.list.len()-1)
    }
}

#[cfg(test)]
mod tests {
    use super::FindSubarry;

    #[test]
    fn merge_sort() {
        let mut insert_sort:FindSubarry<isize>=FindSubarry{
            list:vec![13,-3,-25,20,-3,-16,-23,18,20,-7,12,-5,-22,15,-4,7]
        };
        println!("{:?}",insert_sort.finds());


    }



}