use crate::picture::PictureF32;
use crate::{get_histogram, print_all_diagrams, Histogram, Picture, PictureU8};

#[test]
fn test_creating_new_data_type() {
    pub struct PictureU32 {
        pub lines: u32,
        //height
        pub columns: u32,
        //width
        pub color_channel_count: usize,
        pub data: Vec<u32>, //different data type: u32
    }
    impl Picture for PictureU32 {
        fn to_picture_u8(&self) -> PictureU8 {
            let mut new_data = Vec::<u8>::new();

            //convert each value from [0, u32::MAX] to [0, 255]
            for i in 0..self.data.len() {
                //
                let new_value_in_f32 = ((self.data[i] as f32) / (u32::MAX as f32)) * 255.0;
                new_data.push(new_value_in_f32 as u8);
            }

            PictureU8 {
                lines: self.lines,
                columns: self.columns,
                color_channel_count: self.color_channel_count,
                data: new_data,
            }
        }

        fn to_picture_f32(&self) -> PictureF32 {
            let mut new_data = Vec::<f32>::new();

            //convert each value from [0, u32::MAX] to [0.0, 1.0]
            for i in 0..self.data.len() {
                new_data.push((self.data[i] as f32) / (u32::MAX as f32));
            }

            PictureF32 {
                lines: self.lines,
                columns: self.columns,
                color_channel_count: self.color_channel_count,
                data: new_data,
            }
        }
    }

    let pic_u32 = PictureU32 {
        lines: 1,
        columns: 3,
        color_channel_count: 3,
        data: vec![
            123_456,
            128,
            0,
            210_000_000,
            0,
            0,
            456_234,
            90_000,
            2_123_000_333,
        ],
    };

    let expected_result = vec![
        Histogram {
            bins: vec![3, 0, 0, 0, 0],
        },
        Histogram {
            bins: vec![3, 0, 0, 0, 0],
        },
        Histogram {
            bins: vec![2, 0, 1, 0, 0],
        },
    ];

    // assert with get_histogram as a random sample to ensure functionality
    // of the Picture-typed parameter
    assert_eq!(expected_result, get_histogram(&pic_u32));
}
