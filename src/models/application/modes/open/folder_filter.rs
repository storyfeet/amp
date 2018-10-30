use std::collections::BTreeMap;
use std::path::{PathBuf,Path};

pub struct ShrinkPath<'a>{
    p:&'a Path,
    shrunk:bool,
}

impl ShrinkPath{
    pub fn new(p:&Path)->ShrinkPath{
        ShrinkPath{
            p:p,
            shrunk:false;
        }
    }
    pub fn parent(&mut self)->&Path{
        let p2 = self.p.parent().unwrap_or(self.p);
        self.p = p2;
        p2
    }
}

pub fn split_gt(s:&str)->(&str,&str){
    if s.len()==0 {
        return (s,"")
    }
    if s.chars().next() == Some('>') {
        return (&s.trim_matches('>'),s.split(' ').next().unwrap_or("").trim_matches('>'));
    }
    (s,"")
}

fn no_dot(p:&Path)->&Path{
    if p.ends_with("..."){
        return p.parent().expect("Ends with ..., must have parent");
    }
    p
}
fn parent_dot(mut p:&Path)->Option<PathBuf>{
    p = no_dot(p);
    p = p.parent()?;
    let mut res = PathBuf::from(p);
    res.push("...");
    Some(res)
}



//Limit find the longest common path
pub fn search_as_folders<'a,IT>(it:IT,f_root:&str)->Option<Vec<PathBuf>>
    where IT:IntoIterator<Item=&'a Path>
{
    let mut common = None;
    let mut deeper = BTreeMap::new();

    for v in it {
        if ! v.starts_with(f_root){ continue }

        match common{
            None=>{
                common = Some(PathBuf::from(v));
                deeper.insert(PathBuf::from(v),());
                continue;
            },
            Some(ref mut common)=>{
                let mut v = PathBuf::from(v);
                while ! &v.starts_with(&common) {
                    *common = common.parent()?.into();
                }
                if v == *common {
                    continue
                }
                while no_dot(&v).parent() != Some(&common){
                    v = parent_dot(&v)?;
                }
                deeper.insert(v,());
            }
        }
    }

    let mut d2 = BTreeMap::new();
    if let Some(comm) = common{
        for (mut item,_) in deeper{
            if no_dot(&item) == &comm{
                d2.insert(item,());
                continue;
            }
            while no_dot(&item).parent() != Some(&comm){
                item = parent_dot(&item)?;
            }
            d2.insert(item,());
        }
    }
    Some(d2.into_iter().map(|(k,_)|k).collect())
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn folder_filter_test1(){
        let vals = vec![
            PathBuf::from("/hello/world/buddy"),
            PathBuf::from("/hello/venus/buddy"),
            PathBuf::from("/hello/buddy"),
        ];
        let v2:Vec<&Path> = (&vals).into_iter().map(|x|x as &Path).collect();
        let r = search_as_folders(v2,"").unwrap(); 
        assert_eq!(r,vec![
                  PathBuf::from("/hello/buddy"),
                  PathBuf::from("/hello/venus/..."),
                  PathBuf::from("/hello/world/..."),
                  ]);
    }        

    #[test]
    fn folder_filter_test2(){
        let vals = vec![
            PathBuf::from("/hello/world/buddy"),
            PathBuf::from("/hello/venus/buddy"),
            PathBuf::from("/hello/buddy"),
            PathBuf::from("/group/buddy"),
        ];
        let v2:Vec<&Path> = (&vals).into_iter().map(|x|x as &Path).collect();
        let r = search_as_folders(v2,"/group").unwrap(); 
        assert_eq!(r,vec![
                  PathBuf::from("/group/buddy"),
                  //PathBuf::from("/hello/..."),
                  ]);
    }        

    #[test]
    fn split_gt_test(){
        assert_eq!(split_gt(""),("",""));
        assert_eq!(split_gt("hello goodbye"),("hello goodbye",""));
        assert_eq!(split_gt(">hello goodbye"),("hello goodbye","hello"));
    }

}
