use groth16_solana::groth16::Groth16Verifyingkey;

pub const RISC0_VERIFYINGKEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 5,

    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        45, 160, 168, 242, 218, 168, 73, 106, 143, 63, 204, 39, 190, 58, 111, 149, 208, 253, 25,
        76, 16, 76, 194, 86, 26, 233, 69, 185, 239, 98, 178, 21, 46, 118, 143, 187, 74, 121, 204,
        102, 124, 36, 145, 59, 142, 166, 19, 9, 73, 250, 238, 27, 78, 249, 228, 146, 221, 8, 58,
        112, 90, 126, 202, 227, 16, 40, 187, 56, 229, 15, 36, 86, 119, 153, 60, 217, 68, 179, 255,
        116, 175, 123, 249, 247, 82, 124, 238, 142, 96, 76, 98, 32, 168, 72, 197, 190, 43, 73, 47,
        201, 72, 154, 201, 214, 4, 31, 56, 54, 186, 249, 58, 16, 159, 146, 83, 82, 157, 128, 18,
        224, 28, 224, 17, 195, 22, 200, 59, 230,
    ],

    vk_ic: &[
        [
            10, 41, 10, 55, 228, 68, 149, 248, 186, 111, 121, 183, 144, 162, 98, 167, 49, 48, 165,
            90, 34, 142, 40, 142, 241, 141, 78, 163, 34, 190, 87, 187, 30, 4, 201, 134, 89, 159,
            197, 194, 126, 250, 30, 150, 51, 91, 20, 44, 154, 247, 198, 101, 219, 140, 194, 190,
            236, 45, 244, 106, 65, 58, 141, 15,
        ],
        [
            42, 72, 200, 4, 252, 10, 89, 133, 48, 243, 104, 206, 195, 142, 79, 250, 28, 40, 234,
            73, 60, 89, 116, 121, 92, 183, 83, 117, 44, 220, 241, 21, 1, 10, 74, 111, 126, 171,
            165, 235, 4, 79, 202, 80, 231, 192, 191, 20, 24, 85, 253, 199, 153, 187, 168, 223, 134,
            19, 251, 251, 143, 180, 246, 202,
        ],
        [
            21, 169, 212, 69, 42, 151, 73, 208, 140, 241, 88, 134, 255, 61, 189, 182, 213, 36, 28,
            178, 187, 51, 5, 248, 53, 145, 97, 101, 62, 155, 215, 179, 34, 50, 126, 116, 169, 97,
            74, 225, 39, 22, 135, 72, 207, 59, 231, 228, 177, 185, 126, 149, 182, 33, 0, 204, 97,
            77, 227, 64, 188, 155, 131, 131,
        ],
        [
            10, 87, 146, 185, 89, 30, 198, 197, 143, 226, 46, 13, 228, 124, 157, 90, 80, 165, 19,
            5, 195, 22, 205, 83, 206, 93, 44, 250, 253, 73, 154, 70, 47, 87, 198, 127, 73, 199, 49,
            132, 47, 103, 103, 73, 230, 134, 224, 246, 90, 57, 124, 207, 10, 201, 141, 206, 221,
            190, 42, 221, 230, 128, 139, 1,
        ],
        [
            38, 245, 245, 240, 206, 161, 93, 215, 183, 147, 16, 9, 94, 96, 82, 182, 46, 238, 43,
            91, 252, 45, 33, 37, 128, 40, 1, 223, 23, 189, 137, 21, 23, 81, 2, 77, 188, 45, 236,
            116, 112, 97, 121, 7, 221, 241, 154, 125, 72, 96, 27, 134, 185, 55, 187, 225, 6, 85,
            121, 193, 30, 89, 119, 135,
        ],
    ],
};