
use std::env;
use std::path::PathBuf;

use pdf::file::File;
use pdf::error::PdfError;
use pdf::content::*;
// use pdf::object::*;
// use pdf::build::*;


pub fn read() -> Result<(), PdfError> {
    let path = PathBuf::from(env::args_os().nth(1).expect("no file given"));
    let mut file = File::<Vec<u8>>::open(&path).unwrap();
    for page in file.pages().take(1) {
        let page = page.unwrap();
        if let Some(ref c) = page.contents {
            println!("{:?}", c);
            println!("{:?}", &c.operations(&file));
        }
        let _content = Content::from_ops(vec![
            Op::MoveTo { p: Point { x: 100., y: 100. } },
            Op::LineTo { p: Point { x: 100., y: 200. } },
            Op::LineTo { p: Point { x: 200., y: 100. } },
            Op::LineTo { p: Point { x: 200., y: 200. } },
            Op::Close,
            Op::Stroke,
        ]);
        // let mut new_page = PageBuilder::from_page(&page)?;
        // for s in new_page.content.as_mut().iter_mut().flat_map(|c| c.parts.iter_mut()) {
        //     *s = Stream::new((), s.data(&file)?);
        // }
        // pages.push(new_page);
    }
    // let catalog = CatalogBuilder::from_pages(pages)
    //     .build(&mut file).unwrap();
    // file.update_catalog(catalog)?;
    // file.save_to(path.with_extension("modified.pdf"))?;
    Ok(())
}


