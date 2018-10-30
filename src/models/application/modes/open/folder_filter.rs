use std::collections::BTreeMap;
use std::path::{PathBuf,Path};

#[derive(Ord,Eq,PartialOrd,PartialEq)]
pub struct ShrinkPath<'a>{
    p:&'a Path,
    shrunk:bool,
}

impl<'a> ShrinkPath<'a>{
    pub fn new(p:&'a Path)->ShrinkPath<'a>{
        ShrinkPath{
            p:p,
            shrunk:false,
        }
    }

    pub fn parent(&self)->&Path{
        let p2 = self.p.parent().unwrap_or(self.p);
        p2
    }

    pub fn to_parent(&mut self){
        self.p = self.p.parent().unwrap_or(self.p);
        self.shrunk = true;
    }

    pub fn to_buf(&self)->PathBuf{
        let mut res = PathBuf::from(self.p);
        if self.shrunk {
            res.push("...");
        }
        res
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
                common = Some(v);
                deeper.insert(ShrinkPath::new(v),());
                continue;
            },
            Some(ref mut common)=>{
                let mut v = ShrinkPath::new(v);
                while ! v.p.starts_with(&common) {
                    *common = common.parent()?;
                }
                if v.p == *common {
                    continue
                }
                while v.parent() != *common{
                    v.to_parent();
                }
                deeper.insert(v,());
            }
        }
    }

    let mut d2 = BTreeMap::new();
    if let Some(ref comm) = common{
        for (mut sv, _) in deeper{
            if sv.p == *comm{
                d2.insert(sv.to_buf(),());
                continue;
            }
            while sv.parent() != *comm{
                sv.to_parent();
            }

            d2.insert(sv.to_buf(),());
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
