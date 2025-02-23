mod coding;
mod errors;
mod models;
mod utils;

use coding::huffman::{HuffmanCoding, HuffmanEncoding, StandardHuffmanCoding};
use errors::compression_error::CompressionError;

pub fn compress(text: &String) -> Result<String, CompressionError> {
    match <StandardHuffmanCoding as HuffmanCoding>::encode(&text) {
        Ok(encoding_result) => Ok(encoding_result.serialize()),
        Err(e) => Err(CompressionError::cannot_compress_text(e.msg)),
    }
}

pub fn decompress(text: &String) -> Result<String, CompressionError> {
    match HuffmanEncoding::deserialize(text.to_string()) {
        Ok(encoding) => {
            let tree = encoding.get_huffman_tree().clone();
            let bin_vec = encoding.get_encoded_vec().clone();

            match <StandardHuffmanCoding as HuffmanCoding>::decode(tree, bin_vec) {
                Ok(decoding) => Ok(decoding.get_decoded_text().clone()),
                Err(e) => Err(CompressionError::cannot_decompress_text(e.msg)),
            }
        }
        Err(e) => Err(CompressionError::cannot_decompress_text(e.msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let text = "Huffman-ft-uyen".to_string();
        let result = compress(&text);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            "1-29-001f01n1-0001y1t1u001a1H01m1em\u{7}1\u{19}\u{17}1t"
        );
    }

    #[test]
    fn test_decompress_successful() {
        let text = "1-29-001f01n1-0001y1t1u001a1H01m1em\u{7}1\u{19}\u{17}1t".to_string();
        let result = decompress(&text);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, "Huffman-ft-uyen");
    }

    #[test]
    fn test_decompress_failed() {
        let text = "1-29-001f01n1-0001y1t1u001a1H01m1em\u{7}1\u{19}\u{17}1t6457".to_string();
        let result = decompress(&text);

        assert!(result.is_err());
    }

    #[test]
    fn test_full_flow() {
        let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla id tortor at ex pellentesque consectetur. Pellentesque eget commodo ex. Quisque et luctus sem. Quisque a massa nulla. Suspendisse aliquam, arcu et vulputate consectetur, risus erat consequat nibh, vel placerat lorem neque quis dolor. Curabitur non odio et augue volutpat convallis non ac dolor. Quisque venenatis, ex in egestas aliquet, velit lectus molestie nibh, eget eleifend ex sapien non elit. Phasellus vitae justo velit. Suspendisse et massa eu mi ullamcorper hendrerit vel ut sapien. Etiam euismod dapibus enim, tincidunt efficitur enim laoreet euismod. Maecenas interdum pulvinar odio sed luctus.

In ut dui metus. Maecenas semper rhoncus accumsan. Cras eleifend ex ac risus blandit placerat. Suspendisse semper mollis mauris. Nullam velit ipsum, egestas quis lacus eu, dictum fermentum mi. Aliquam sit amet luctus mi, sit amet lobortis mi. Praesent non lectus porta, interdum leo quis, accumsan nunc. Duis vehicula sodales leo. Proin nulla enim, lobortis at augue laoreet, luctus tempor diam. Vestibulum in sem at dui pretium bibendum. Nulla ullamcorper sollicitudin condimentum. Donec dignissim risus molestie, auctor felis id, porta mauris. Sed dapibus pulvinar mi, rutrum imperdiet libero ornare in. Proin varius mi sit amet enim rhoncus, sed euismod lacus elementum. Aenean ullamcorper at dolor vitae consectetur. Vivamus ac semper purus, quis lobortis odio.

Nam condimentum tincidunt ante, sollicitudin dapibus justo. Proin lacinia sit amet sapien non ultrices. Fusce dolor risus, porttitor non aliquam nec, rutrum vel arcu. Phasellus pharetra egestas dui et facilisis. Quisque in ex et erat tempus tristique. Sed non posuere felis. Phasellus sit amet tristique mi. Proin at nibh ipsum. Nam ac dui ullamcorper purus rutrum consequat nec nec nibh. Nunc ac lectus sed nulla volutpat ultricies non sit amet ante. Pellentesque semper tempus facilisis. Cras id leo vestibulum, placerat libero vitae, gravida mi. Nam maximus scelerisque dignissim. Nam turpis metus, placerat at ultrices nec, dapibus sit amet ante. Fusce at vehicula massa. Aenean dignissim vehicula faucibus.

Aenean in blandit nisl, eu facilisis nisi. Quisque ac libero vel libero placerat ornare. Ut ac convallis libero. Vivamus ac arcu tempor felis cursus consectetur. Pellentesque consectetur eleifend ex, eget scelerisque metus tincidunt sit amet. Suspendisse sodales molestie orci, ac tristique mauris auctor quis. In accumsan mollis nisl, accumsan vehicula ligula luctus quis. Proin laoreet nibh luctus velit auctor aliquam. Duis ultrices massa justo, suscipit fringilla libero fringilla sed. Integer laoreet lacus vel fringilla bibendum. Suspendisse potenti. Nulla eget aliquam ipsum. Donec vehicula placerat ullamcorper. Phasellus vitae felis facilisis, venenatis ligula et, elementum sapien.

Praesent eros urna, imperdiet sit amet laoreet eu, fringilla sed dui. Aenean pharetra iaculis orci nec viverra. Donec id sem vel quam efficitur placerat. Cras et ligula porta, interdum risus id, egestas sapien. Curabitur mi lorem, dignissim at risus ac, faucibus malesuada erat. Praesent euismod nibh nulla, non mattis nulla tincidunt vel. Phasellus viverra commodo magna et tempor. Nam interdum a nisi vel mattis. Aliquam vitae erat eget neque condimentum condimentum. Quisque gravida tortor id est pretium, eu porta lacus mattis. Aenean nec risus non ipsum aliquam cursus. Proin elit massa, finibus non tempor et, dignissim ut massa. Sed dui nibh, tincidunt in pellentesque a, fermentum sit amet orci.".to_string();
        let compressing_result = compress(&text);
        let compressing_text = compressing_result.unwrap();
        let decompressing_result = decompress(&compressing_text);

        println!("Original length: {}", text.len());
        println!("New length: {}", compressing_text.len());

        assert_eq!(decompressing_result.unwrap(), text);
    }
}
