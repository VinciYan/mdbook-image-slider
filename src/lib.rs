use mdbook::book::{Book};
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
pub struct ImageViewerPreprocessor;

impl Preprocessor for ImageViewerPreprocessor {
    fn name(&self) -> &str {
        "image-slider"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                let mut new_content = String::new();
                // 在每个章节的开始注入 CSS
                new_content.push_str(include_str!("css_template.html"));
                new_content.push('\n');
                
                // 处理章节内容
                new_content.push_str(&self.process_chapter(&chapter.content));
                new_content.push('\n');
                
                // 在每个章节的结束注入 JS
                new_content.push_str(include_str!("script_template.html"));
                chapter.content = new_content;
            }
        });
        Ok(book)
    }
}

impl ImageViewerPreprocessor {
    fn process_chapter(&self, content: &str) -> String {
        let mut output = String::new();
        let mut in_viewer = false;
        let mut images = Vec::new();

        for line in content.lines() {
            if line.trim() == "{{image-slider}}" {
                in_viewer = true;
                continue;
            }
            if line.trim() == "{{/image-slider}}" {
                in_viewer = false;
                output.push_str(&self.create_image_viewer(&images));
                images.clear();
                continue;
            }
            if in_viewer {
                if let Some(image_info) = line.trim().strip_prefix("image: ") {
                    let parts: Vec<&str> = image_info.splitn(2, " caption: ").collect();
                    if parts.len() == 2 {
                        images.push((parts[0].to_string(), parts[1].to_string()));
                    } else {
                        images.push((parts[0].to_string(), String::new()));
                    }
                }
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }
        output
    }

    fn create_image_viewer(&self, images: &[(String, String)]) -> String {
        let viewer_id = format!("viewer-{}", uuid::Uuid::new_v4());
        let images_html = images
            .iter()
            .enumerate()
            .map(|(i, (path, caption))| format!(
                r#"<div class="viewer-slide" style="display: {};">
                    <img src="{}" class="viewer-image" onclick="openModal(this.src)">
                    <p class="viewer-caption">{}</p>
                </div>"#,
                if i == 0 { "block" } else { "none" },
                path,
                caption
            ))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"
<div class="image-viewer" id="{}">
    <div class="viewer-container">
        {}
    </div>
    <div class="image-counter">1/{}</div>
    <button class="viewer-btn prev-btn" style="display: none;">&lt;</button>
    <button class="viewer-btn next-btn" style="display: none;">&gt;</button>
</div>
<div id="imageModal" class="modal">
    <span class="close" onclick="closeModal()">&times;</span>
    <div class="modal-content">
        <img id="modalImage">
    </div>
</div>
"#,
            viewer_id,
            images_html,
            images.len()
        )
    }
}