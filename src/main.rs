use std::{fs::File, io::{BufReader, BufWriter, Read, Write}};

use clap::Parser;
use docx_rs::{read_docx, Document, DocumentChild, Docx, Paragraph, ParagraphChild, Run, RunChild, Style, Table, TableCellContent, TableChild, TableRowChild};


/// 根据请求地址获取nacos配置文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 算法类型
    #[arg(short, long, default_value = "read",)]
    model: String,

    /// 文件路径(不包含文件名)
    #[arg(short='p', long ,default_value = "./")]
    path: String,
}


fn main(){
   let args = Args::parse();
   match &args.path as &str{

     "./"=>{
        //let path = std::path::Path::new("./hello.docx");
        //let file = std::fs::File::create(&path).unwrap();

        //let file = File::open("/Users/martin/Documents/电信集成/海关项目/甲方提交的材料/海关智慧监管平台深化设计方案20240128/01海关智慧监管平台深化设计方案-总册20240128.docx").unwrap();
        let file = File::open("/Users/martin/Downloads/workspace/rust/zbox-find/hello.docx").unwrap();
        let mut reader = BufReader::new(file);

        let mut buf = vec![];
        let _ = reader.read_to_end(&mut buf);
       
       // file.read_to_end(&mut buf).unwrap();
        let res: Docx = read_docx(&buf).unwrap();

        //print!("has_numbering: {}",res.document.has_numbering);
        //print!("section_property: {:?}",res.document.section_property);

    
        let children = res.document.children;
        for i in children {
            match i {
                DocumentChild::Paragraph(para) => {
                    print_paragraph(para);
                }
                DocumentChild::Table(s) => {
                   // print_table(s);
                }
                DocumentChild::BookmarkStart(s) => {
                    println!("s3 => {:?}\n", s);
                }
                DocumentChild::BookmarkEnd(s) => {
                    println!("s4 => {:?}\n", s);
                }
                DocumentChild::CommentStart(s) => {
                    println!("s5 => {:?}\n", s);
                }
                DocumentChild::CommentEnd(s) => {
                    println!("s6 => {:?}\n", s);
                }
                DocumentChild::StructuredDataTag(s) => {
                    println!("s7 => {:?}\n", s);
                }
                DocumentChild::TableOfContents(s) => {
                    println!("s8 => {:?}\n", s);
                }
            }
        }
    
        //bw.flush();

    }
    _ => {
        println!("-h 查看帮助");
    },
}




fn print_paragraph(paragraph: Box<Paragraph>) {
    //println!("get a aragraph => {:?}", paragraph);
    println!("id => {:?}", paragraph.id);
    println!("property => {:?}", paragraph.property);
    println!("has_numbering => {:?}", paragraph.has_numbering);
 
   

   let mut one_paragraph =vec![];
   

    for ele in paragraph.children {
        match ele {
            ParagraphChild::Run(r) => {
                for ele2 in r.children {
                    match ele2 {
                        RunChild::Text(t) => {
                          // println!("text: {}", t.text);
                           one_paragraph.push(t.text);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    let content = one_paragraph.concat();
    print!("{}\n", content);
   // bw.write_all(content.as_bytes());
}


}
