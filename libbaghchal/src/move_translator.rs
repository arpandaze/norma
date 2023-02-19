use crate::types::Move;
use crate::BaghchalRS;

impl BaghchalRS {
    pub fn i2m_goat(index: usize) -> Move {
        match index {
            0 => return (Some([0, 0]), [0, 1]),
            1 => return (Some([0, 0]), [1, 0]),
            2 => return (Some([0, 0]), [1, 1]),
            3 => return (Some([0, 1]), [0, 0]),
            4 => return (Some([0, 1]), [0, 2]),
            5 => return (Some([0, 1]), [1, 1]),
            6 => return (Some([0, 2]), [0, 1]),
            7 => return (Some([0, 2]), [0, 3]),
            8 => return (Some([0, 2]), [1, 1]),
            9 => return (Some([0, 2]), [1, 2]),
            10 => return (Some([0, 2]), [1, 3]),
            11 => return (Some([0, 3]), [0, 2]),
            12 => return (Some([0, 3]), [0, 4]),
            13 => return (Some([0, 3]), [1, 3]),
            14 => return (Some([0, 4]), [0, 3]),
            15 => return (Some([0, 4]), [1, 3]),
            16 => return (Some([0, 4]), [1, 4]),
            17 => return (Some([1, 0]), [0, 0]),
            18 => return (Some([1, 0]), [1, 1]),
            19 => return (Some([1, 0]), [2, 0]),
            20 => return (Some([1, 1]), [0, 0]),
            21 => return (Some([1, 1]), [0, 1]),
            22 => return (Some([1, 1]), [0, 2]),
            23 => return (Some([1, 1]), [1, 0]),
            24 => return (Some([1, 1]), [1, 2]),
            25 => return (Some([1, 1]), [2, 0]),
            26 => return (Some([1, 1]), [2, 1]),
            27 => return (Some([1, 1]), [2, 2]),
            28 => return (Some([1, 2]), [0, 2]),
            29 => return (Some([1, 2]), [1, 1]),
            30 => return (Some([1, 2]), [1, 3]),
            31 => return (Some([1, 2]), [2, 2]),
            32 => return (Some([1, 3]), [0, 2]),
            33 => return (Some([1, 3]), [0, 3]),
            34 => return (Some([1, 3]), [0, 4]),
            35 => return (Some([1, 3]), [1, 2]),
            36 => return (Some([1, 3]), [1, 4]),
            37 => return (Some([1, 3]), [2, 2]),
            38 => return (Some([1, 3]), [2, 3]),
            39 => return (Some([1, 3]), [2, 4]),
            40 => return (Some([1, 4]), [0, 4]),
            41 => return (Some([1, 4]), [1, 3]),
            42 => return (Some([1, 4]), [2, 4]),
            43 => return (Some([2, 0]), [1, 0]),
            44 => return (Some([2, 0]), [1, 1]),
            45 => return (Some([2, 0]), [2, 1]),
            46 => return (Some([2, 0]), [3, 0]),
            47 => return (Some([2, 0]), [3, 1]),
            48 => return (Some([2, 1]), [1, 1]),
            49 => return (Some([2, 1]), [2, 0]),
            50 => return (Some([2, 1]), [2, 2]),
            51 => return (Some([2, 1]), [3, 1]),
            52 => return (Some([2, 2]), [1, 1]),
            53 => return (Some([2, 2]), [1, 2]),
            54 => return (Some([2, 2]), [1, 3]),
            55 => return (Some([2, 2]), [2, 1]),
            56 => return (Some([2, 2]), [2, 3]),
            57 => return (Some([2, 2]), [3, 1]),
            58 => return (Some([2, 2]), [3, 2]),
            59 => return (Some([2, 2]), [3, 3]),
            60 => return (Some([2, 3]), [1, 3]),
            61 => return (Some([2, 3]), [2, 2]),
            62 => return (Some([2, 3]), [2, 4]),
            63 => return (Some([2, 3]), [3, 3]),
            64 => return (Some([2, 4]), [1, 3]),
            65 => return (Some([2, 4]), [1, 4]),
            66 => return (Some([2, 4]), [2, 3]),
            67 => return (Some([2, 4]), [3, 3]),
            68 => return (Some([2, 4]), [3, 4]),
            69 => return (Some([3, 0]), [2, 0]),
            70 => return (Some([3, 0]), [3, 1]),
            71 => return (Some([3, 0]), [4, 0]),
            72 => return (Some([3, 1]), [2, 0]),
            73 => return (Some([3, 1]), [2, 1]),
            74 => return (Some([3, 1]), [2, 2]),
            75 => return (Some([3, 1]), [3, 0]),
            76 => return (Some([3, 1]), [3, 2]),
            77 => return (Some([3, 1]), [4, 0]),
            78 => return (Some([3, 1]), [4, 1]),
            79 => return (Some([3, 1]), [4, 2]),
            80 => return (Some([3, 2]), [2, 2]),
            81 => return (Some([3, 2]), [3, 1]),
            82 => return (Some([3, 2]), [3, 3]),
            83 => return (Some([3, 2]), [4, 2]),
            84 => return (Some([3, 3]), [2, 2]),
            85 => return (Some([3, 3]), [2, 3]),
            86 => return (Some([3, 3]), [2, 4]),
            87 => return (Some([3, 3]), [3, 2]),
            88 => return (Some([3, 3]), [3, 4]),
            89 => return (Some([3, 3]), [4, 2]),
            90 => return (Some([3, 3]), [4, 3]),
            91 => return (Some([3, 3]), [4, 4]),
            92 => return (Some([3, 4]), [2, 4]),
            93 => return (Some([3, 4]), [3, 3]),
            94 => return (Some([3, 4]), [4, 4]),
            95 => return (Some([4, 0]), [3, 0]),
            96 => return (Some([4, 0]), [3, 1]),
            97 => return (Some([4, 0]), [4, 1]),
            98 => return (Some([4, 1]), [3, 1]),
            99 => return (Some([4, 1]), [4, 0]),
            100 => return (Some([4, 1]), [4, 2]),
            101 => return (Some([4, 2]), [3, 1]),
            102 => return (Some([4, 2]), [3, 2]),
            103 => return (Some([4, 2]), [3, 3]),
            104 => return (Some([4, 2]), [4, 1]),
            105 => return (Some([4, 2]), [4, 3]),
            106 => return (Some([4, 3]), [3, 3]),
            107 => return (Some([4, 3]), [4, 2]),
            108 => return (Some([4, 3]), [4, 4]),
            109 => return (Some([4, 4]), [3, 3]),
            110 => return (Some([4, 4]), [3, 4]),
            111 => return (Some([4, 4]), [4, 3]),
            _ => panic!("Invalid value to `i2m_goat`"),
        }
    }

    pub fn i2m_placement(index: usize) -> Move {
        match index {
            0 => return (None, [0, 1]),
            1 => return (None, [1, 0]),
            2 => return (None, [1, 1]),
            3 => return (None, [0, 0]),
            4 => return (None, [0, 2]),
            5 => return (None, [0, 3]),
            6 => return (None, [1, 2]),
            7 => return (None, [1, 3]),
            8 => return (None, [0, 4]),
            9 => return (None, [1, 4]),
            10 => return (None, [2, 0]),
            11 => return (None, [2, 1]),
            12 => return (None, [2, 2]),
            13 => return (None, [2, 3]),
            14 => return (None, [2, 4]),
            15 => return (None, [3, 0]),
            16 => return (None, [3, 1]),
            17 => return (None, [3, 2]),
            18 => return (None, [3, 3]),
            19 => return (None, [3, 4]),
            20 => return (None, [4, 0]),
            21 => return (None, [4, 1]),
            22 => return (None, [4, 2]),
            23 => return (None, [4, 3]),
            24 => return (None, [4, 4]),
            _ => panic!("Invalid value to `i2m_placement`"),
        }
    }
    pub fn i2m_tiger(index: usize) -> Move {
        match index {
            0 => return (Some([0, 1]), [0, 2]),
            1 => return (Some([0, 1]), [1, 1]),
            2 => return (Some([0, 2]), [0, 1]),
            3 => return (Some([0, 2]), [0, 3]),
            4 => return (Some([0, 2]), [1, 1]),
            5 => return (Some([0, 2]), [1, 2]),
            6 => return (Some([0, 2]), [1, 3]),
            7 => return (Some([0, 3]), [0, 2]),
            8 => return (Some([0, 3]), [0, 4]),
            9 => return (Some([0, 3]), [1, 3]),
            10 => return (Some([0, 4]), [0, 3]),
            11 => return (Some([0, 4]), [1, 3]),
            12 => return (Some([0, 4]), [1, 4]),
            13 => return (Some([1, 0]), [1, 1]),
            14 => return (Some([1, 0]), [2, 0]),
            15 => return (Some([1, 1]), [0, 1]),
            16 => return (Some([1, 1]), [0, 2]),
            17 => return (Some([1, 1]), [1, 0]),
            18 => return (Some([1, 1]), [1, 2]),
            19 => return (Some([1, 1]), [2, 0]),
            20 => return (Some([1, 1]), [2, 1]),
            21 => return (Some([1, 1]), [2, 2]),
            22 => return (Some([1, 2]), [0, 2]),
            23 => return (Some([1, 2]), [1, 1]),
            24 => return (Some([1, 2]), [1, 3]),
            25 => return (Some([1, 2]), [2, 2]),
            26 => return (Some([1, 3]), [0, 2]),
            27 => return (Some([1, 3]), [0, 3]),
            28 => return (Some([1, 3]), [0, 4]),
            29 => return (Some([1, 3]), [1, 2]),
            30 => return (Some([1, 3]), [1, 4]),
            31 => return (Some([1, 3]), [2, 2]),
            32 => return (Some([1, 3]), [2, 3]),
            33 => return (Some([1, 3]), [2, 4]),
            34 => return (Some([1, 4]), [0, 4]),
            35 => return (Some([1, 4]), [1, 3]),
            36 => return (Some([1, 4]), [2, 4]),
            37 => return (Some([2, 0]), [1, 0]),
            38 => return (Some([2, 0]), [1, 1]),
            39 => return (Some([2, 0]), [2, 1]),
            40 => return (Some([2, 0]), [3, 0]),
            41 => return (Some([2, 0]), [3, 1]),
            42 => return (Some([2, 1]), [1, 1]),
            43 => return (Some([2, 1]), [2, 0]),
            44 => return (Some([2, 1]), [2, 2]),
            45 => return (Some([2, 1]), [3, 1]),
            46 => return (Some([2, 2]), [1, 1]),
            47 => return (Some([2, 2]), [1, 2]),
            48 => return (Some([2, 2]), [1, 3]),
            49 => return (Some([2, 2]), [2, 1]),
            50 => return (Some([2, 2]), [2, 3]),
            51 => return (Some([2, 2]), [3, 1]),
            52 => return (Some([2, 2]), [3, 2]),
            53 => return (Some([2, 2]), [3, 3]),
            54 => return (Some([2, 3]), [1, 3]),
            55 => return (Some([2, 3]), [2, 2]),
            56 => return (Some([2, 3]), [2, 4]),
            57 => return (Some([2, 3]), [3, 3]),
            58 => return (Some([2, 4]), [1, 3]),
            59 => return (Some([2, 4]), [1, 4]),
            60 => return (Some([2, 4]), [2, 3]),
            61 => return (Some([2, 4]), [3, 3]),
            62 => return (Some([2, 4]), [3, 4]),
            63 => return (Some([3, 0]), [2, 0]),
            64 => return (Some([3, 0]), [3, 1]),
            65 => return (Some([3, 0]), [4, 0]),
            66 => return (Some([3, 1]), [2, 0]),
            67 => return (Some([3, 1]), [2, 1]),
            68 => return (Some([3, 1]), [2, 2]),
            69 => return (Some([3, 1]), [3, 0]),
            70 => return (Some([3, 1]), [3, 2]),
            71 => return (Some([3, 1]), [4, 0]),
            72 => return (Some([3, 1]), [4, 1]),
            73 => return (Some([3, 1]), [4, 2]),
            74 => return (Some([3, 2]), [2, 2]),
            75 => return (Some([3, 2]), [3, 1]),
            76 => return (Some([3, 2]), [3, 3]),
            77 => return (Some([3, 2]), [4, 2]),
            78 => return (Some([3, 3]), [2, 2]),
            79 => return (Some([3, 3]), [2, 3]),
            80 => return (Some([3, 3]), [2, 4]),
            81 => return (Some([3, 3]), [3, 2]),
            82 => return (Some([3, 3]), [3, 4]),
            83 => return (Some([3, 3]), [4, 2]),
            84 => return (Some([3, 3]), [4, 3]),
            85 => return (Some([3, 3]), [4, 4]),
            86 => return (Some([3, 4]), [2, 4]),
            87 => return (Some([3, 4]), [3, 3]),
            88 => return (Some([3, 4]), [4, 4]),
            89 => return (Some([4, 0]), [3, 0]),
            90 => return (Some([4, 0]), [3, 1]),
            91 => return (Some([4, 0]), [4, 1]),
            92 => return (Some([4, 1]), [3, 1]),
            93 => return (Some([4, 1]), [4, 0]),
            94 => return (Some([4, 1]), [4, 2]),
            95 => return (Some([4, 2]), [3, 1]),
            96 => return (Some([4, 2]), [3, 2]),
            97 => return (Some([4, 2]), [3, 3]),
            98 => return (Some([4, 2]), [4, 1]),
            99 => return (Some([4, 2]), [4, 3]),
            100 => return (Some([4, 3]), [3, 3]),
            101 => return (Some([4, 3]), [4, 2]),
            102 => return (Some([4, 3]), [4, 4]),
            103 => return (Some([4, 4]), [3, 3]),
            104 => return (Some([4, 4]), [3, 4]),
            105 => return (Some([4, 4]), [4, 3]),
            106 => return (Some([0, 0]), [0, 2]),
            107 => return (Some([0, 0]), [1, 0]),
            108 => return (Some([0, 0]), [1, 1]),
            109 => return (Some([0, 2]), [0, 0]),
            110 => return (Some([1, 0]), [0, 0]),
            111 => return (Some([1, 1]), [0, 0]),
            112 => return (Some([0, 0]), [0, 1]),
            113 => return (Some([0, 1]), [0, 0]),
            114 => return (Some([0, 1]), [0, 3]),
            115 => return (Some([0, 3]), [0, 1]),
            116 => return (Some([0, 2]), [0, 4]),
            117 => return (Some([0, 4]), [0, 2]),
            118 => return (Some([0, 0]), [2, 0]),
            119 => return (Some([2, 0]), [0, 0]),
            120 => return (Some([0, 0]), [2, 2]),
            121 => return (Some([0, 1]), [2, 1]),
            122 => return (Some([0, 2]), [2, 0]),
            123 => return (Some([1, 0]), [1, 2]),
            124 => return (Some([1, 2]), [1, 0]),
            125 => return (Some([2, 0]), [0, 2]),
            126 => return (Some([2, 1]), [0, 1]),
            127 => return (Some([2, 2]), [0, 0]),
            128 => return (Some([0, 2]), [2, 2]),
            129 => return (Some([1, 1]), [1, 3]),
            130 => return (Some([1, 3]), [1, 1]),
            131 => return (Some([2, 2]), [0, 2]),
            132 => return (Some([0, 2]), [2, 4]),
            133 => return (Some([0, 3]), [2, 3]),
            134 => return (Some([0, 4]), [2, 2]),
            135 => return (Some([1, 2]), [1, 4]),
            136 => return (Some([1, 4]), [1, 2]),
            137 => return (Some([2, 2]), [0, 4]),
            138 => return (Some([2, 3]), [0, 3]),
            139 => return (Some([2, 4]), [0, 2]),
            140 => return (Some([0, 4]), [2, 4]),
            141 => return (Some([2, 4]), [0, 4]),
            142 => return (Some([1, 0]), [3, 0]),
            143 => return (Some([3, 0]), [1, 0]),
            144 => return (Some([1, 1]), [3, 1]),
            145 => return (Some([2, 0]), [2, 2]),
            146 => return (Some([2, 2]), [2, 0]),
            147 => return (Some([3, 1]), [1, 1]),
            148 => return (Some([1, 1]), [3, 3]),
            149 => return (Some([1, 2]), [3, 2]),
            150 => return (Some([1, 3]), [3, 1]),
            151 => return (Some([2, 1]), [2, 3]),
            152 => return (Some([2, 3]), [2, 1]),
            153 => return (Some([3, 1]), [1, 3]),
            154 => return (Some([3, 2]), [1, 2]),
            155 => return (Some([3, 3]), [1, 1]),
            156 => return (Some([1, 3]), [3, 3]),
            157 => return (Some([2, 2]), [2, 4]),
            158 => return (Some([2, 4]), [2, 2]),
            159 => return (Some([3, 3]), [1, 3]),
            160 => return (Some([1, 4]), [3, 4]),
            161 => return (Some([3, 4]), [1, 4]),
            162 => return (Some([2, 0]), [4, 0]),
            163 => return (Some([4, 0]), [2, 0]),
            164 => return (Some([2, 0]), [4, 2]),
            165 => return (Some([2, 1]), [4, 1]),
            166 => return (Some([2, 2]), [4, 0]),
            167 => return (Some([3, 0]), [3, 2]),
            168 => return (Some([3, 2]), [3, 0]),
            169 => return (Some([4, 0]), [2, 2]),
            170 => return (Some([4, 1]), [2, 1]),
            171 => return (Some([4, 2]), [2, 0]),
            172 => return (Some([2, 2]), [4, 2]),
            173 => return (Some([3, 1]), [3, 3]),
            174 => return (Some([3, 3]), [3, 1]),
            175 => return (Some([4, 2]), [2, 2]),
            176 => return (Some([2, 2]), [4, 4]),
            177 => return (Some([2, 3]), [4, 3]),
            178 => return (Some([2, 4]), [4, 2]),
            179 => return (Some([3, 2]), [3, 4]),
            180 => return (Some([3, 4]), [3, 2]),
            181 => return (Some([4, 2]), [2, 4]),
            182 => return (Some([4, 3]), [2, 3]),
            183 => return (Some([4, 4]), [2, 2]),
            184 => return (Some([2, 4]), [4, 4]),
            185 => return (Some([4, 4]), [2, 4]),
            186 => return (Some([4, 0]), [4, 2]),
            187 => return (Some([4, 2]), [4, 0]),
            188 => return (Some([4, 1]), [4, 3]),
            189 => return (Some([4, 3]), [4, 1]),
            190 => return (Some([4, 2]), [4, 4]),
            191 => return (Some([4, 4]), [4, 2]),
            _ => panic!("Invalid value to `i2m_tiger`"),
        }
    }

    pub fn m2i_goat(move_: Move) -> usize {
        match move_ {
            (Some([0, 0]), [0, 1]) => return 0,
            (Some([0, 0]), [1, 0]) => return 1,
            (Some([0, 0]), [1, 1]) => return 2,
            (Some([0, 1]), [0, 0]) => return 3,
            (Some([0, 1]), [0, 2]) => return 4,
            (Some([0, 1]), [1, 1]) => return 5,
            (Some([0, 2]), [0, 1]) => return 6,
            (Some([0, 2]), [0, 3]) => return 7,
            (Some([0, 2]), [1, 1]) => return 8,
            (Some([0, 2]), [1, 2]) => return 9,
            (Some([0, 2]), [1, 3]) => return 10,
            (Some([0, 3]), [0, 2]) => return 11,
            (Some([0, 3]), [0, 4]) => return 12,
            (Some([0, 3]), [1, 3]) => return 13,
            (Some([0, 4]), [0, 3]) => return 14,
            (Some([0, 4]), [1, 3]) => return 15,
            (Some([0, 4]), [1, 4]) => return 16,
            (Some([1, 0]), [0, 0]) => return 17,
            (Some([1, 0]), [1, 1]) => return 18,
            (Some([1, 0]), [2, 0]) => return 19,
            (Some([1, 1]), [0, 0]) => return 20,
            (Some([1, 1]), [0, 1]) => return 21,
            (Some([1, 1]), [0, 2]) => return 22,
            (Some([1, 1]), [1, 0]) => return 23,
            (Some([1, 1]), [1, 2]) => return 24,
            (Some([1, 1]), [2, 0]) => return 25,
            (Some([1, 1]), [2, 1]) => return 26,
            (Some([1, 1]), [2, 2]) => return 27,
            (Some([1, 2]), [0, 2]) => return 28,
            (Some([1, 2]), [1, 1]) => return 29,
            (Some([1, 2]), [1, 3]) => return 30,
            (Some([1, 2]), [2, 2]) => return 31,
            (Some([1, 3]), [0, 2]) => return 32,
            (Some([1, 3]), [0, 3]) => return 33,
            (Some([1, 3]), [0, 4]) => return 34,
            (Some([1, 3]), [1, 2]) => return 35,
            (Some([1, 3]), [1, 4]) => return 36,
            (Some([1, 3]), [2, 2]) => return 37,
            (Some([1, 3]), [2, 3]) => return 38,
            (Some([1, 3]), [2, 4]) => return 39,
            (Some([1, 4]), [0, 4]) => return 40,
            (Some([1, 4]), [1, 3]) => return 41,
            (Some([1, 4]), [2, 4]) => return 42,
            (Some([2, 0]), [1, 0]) => return 43,
            (Some([2, 0]), [1, 1]) => return 44,
            (Some([2, 0]), [2, 1]) => return 45,
            (Some([2, 0]), [3, 0]) => return 46,
            (Some([2, 0]), [3, 1]) => return 47,
            (Some([2, 1]), [1, 1]) => return 48,
            (Some([2, 1]), [2, 0]) => return 49,
            (Some([2, 1]), [2, 2]) => return 50,
            (Some([2, 1]), [3, 1]) => return 51,
            (Some([2, 2]), [1, 1]) => return 52,
            (Some([2, 2]), [1, 2]) => return 53,
            (Some([2, 2]), [1, 3]) => return 54,
            (Some([2, 2]), [2, 1]) => return 55,
            (Some([2, 2]), [2, 3]) => return 56,
            (Some([2, 2]), [3, 1]) => return 57,
            (Some([2, 2]), [3, 2]) => return 58,
            (Some([2, 2]), [3, 3]) => return 59,
            (Some([2, 3]), [1, 3]) => return 60,
            (Some([2, 3]), [2, 2]) => return 61,
            (Some([2, 3]), [2, 4]) => return 62,
            (Some([2, 3]), [3, 3]) => return 63,
            (Some([2, 4]), [1, 3]) => return 64,
            (Some([2, 4]), [1, 4]) => return 65,
            (Some([2, 4]), [2, 3]) => return 66,
            (Some([2, 4]), [3, 3]) => return 67,
            (Some([2, 4]), [3, 4]) => return 68,
            (Some([3, 0]), [2, 0]) => return 69,
            (Some([3, 0]), [3, 1]) => return 70,
            (Some([3, 0]), [4, 0]) => return 71,
            (Some([3, 1]), [2, 0]) => return 72,
            (Some([3, 1]), [2, 1]) => return 73,
            (Some([3, 1]), [2, 2]) => return 74,
            (Some([3, 1]), [3, 0]) => return 75,
            (Some([3, 1]), [3, 2]) => return 76,
            (Some([3, 1]), [4, 0]) => return 77,
            (Some([3, 1]), [4, 1]) => return 78,
            (Some([3, 1]), [4, 2]) => return 79,
            (Some([3, 2]), [2, 2]) => return 80,
            (Some([3, 2]), [3, 1]) => return 81,
            (Some([3, 2]), [3, 3]) => return 82,
            (Some([3, 2]), [4, 2]) => return 83,
            (Some([3, 3]), [2, 2]) => return 84,
            (Some([3, 3]), [2, 3]) => return 85,
            (Some([3, 3]), [2, 4]) => return 86,
            (Some([3, 3]), [3, 2]) => return 87,
            (Some([3, 3]), [3, 4]) => return 88,
            (Some([3, 3]), [4, 2]) => return 89,
            (Some([3, 3]), [4, 3]) => return 90,
            (Some([3, 3]), [4, 4]) => return 91,
            (Some([3, 4]), [2, 4]) => return 92,
            (Some([3, 4]), [3, 3]) => return 93,
            (Some([3, 4]), [4, 4]) => return 94,
            (Some([4, 0]), [3, 0]) => return 95,
            (Some([4, 0]), [3, 1]) => return 96,
            (Some([4, 0]), [4, 1]) => return 97,
            (Some([4, 1]), [3, 1]) => return 98,
            (Some([4, 1]), [4, 0]) => return 99,
            (Some([4, 1]), [4, 2]) => return 100,
            (Some([4, 2]), [3, 1]) => return 101,
            (Some([4, 2]), [3, 2]) => return 102,
            (Some([4, 2]), [3, 3]) => return 103,
            (Some([4, 2]), [4, 1]) => return 104,
            (Some([4, 2]), [4, 3]) => return 105,
            (Some([4, 3]), [3, 3]) => return 106,
            (Some([4, 3]), [4, 2]) => return 107,
            (Some([4, 3]), [4, 4]) => return 108,
            (Some([4, 4]), [3, 3]) => return 109,
            (Some([4, 4]), [3, 4]) => return 110,
            (Some([4, 4]), [4, 3]) => return 111,
            _ => panic!("Invalid value to `m2i_goat`"),
        }
    }

    pub fn m2i_placement(move_: Move) -> usize {
        match move_ {
            (None, [0, 1]) => return 0,
            (None, [1, 0]) => return 1,
            (None, [1, 1]) => return 2,
            (None, [0, 0]) => return 3,
            (None, [0, 2]) => return 4,
            (None, [0, 3]) => return 5,
            (None, [1, 2]) => return 6,
            (None, [1, 3]) => return 7,
            (None, [0, 4]) => return 8,
            (None, [1, 4]) => return 9,
            (None, [2, 0]) => return 10,
            (None, [2, 1]) => return 11,
            (None, [2, 2]) => return 12,
            (None, [2, 3]) => return 13,
            (None, [2, 4]) => return 14,
            (None, [3, 0]) => return 15,
            (None, [3, 1]) => return 16,
            (None, [3, 2]) => return 17,
            (None, [3, 3]) => return 18,
            (None, [3, 4]) => return 19,
            (None, [4, 0]) => return 20,
            (None, [4, 1]) => return 21,
            (None, [4, 2]) => return 22,
            (None, [4, 3]) => return 23,
            (None, [4, 4]) => return 24,
            _ => panic!("Invalid value to `m2i_placement`"),
        };
    }
    pub fn m2i_tiger(move_: Move) -> usize {
        match move_ {
            (Some([0, 1]), [0, 2]) => return 0,
            (Some([0, 1]), [1, 1]) => return 1,
            (Some([0, 2]), [0, 1]) => return 2,
            (Some([0, 2]), [0, 3]) => return 3,
            (Some([0, 2]), [1, 1]) => return 4,
            (Some([0, 2]), [1, 2]) => return 5,
            (Some([0, 2]), [1, 3]) => return 6,
            (Some([0, 3]), [0, 2]) => return 7,
            (Some([0, 3]), [0, 4]) => return 8,
            (Some([0, 3]), [1, 3]) => return 9,
            (Some([0, 4]), [0, 3]) => return 10,
            (Some([0, 4]), [1, 3]) => return 11,
            (Some([0, 4]), [1, 4]) => return 12,
            (Some([1, 0]), [1, 1]) => return 13,
            (Some([1, 0]), [2, 0]) => return 14,
            (Some([1, 1]), [0, 1]) => return 15,
            (Some([1, 1]), [0, 2]) => return 16,
            (Some([1, 1]), [1, 0]) => return 17,
            (Some([1, 1]), [1, 2]) => return 18,
            (Some([1, 1]), [2, 0]) => return 19,
            (Some([1, 1]), [2, 1]) => return 20,
            (Some([1, 1]), [2, 2]) => return 21,
            (Some([1, 2]), [0, 2]) => return 22,
            (Some([1, 2]), [1, 1]) => return 23,
            (Some([1, 2]), [1, 3]) => return 24,
            (Some([1, 2]), [2, 2]) => return 25,
            (Some([1, 3]), [0, 2]) => return 26,
            (Some([1, 3]), [0, 3]) => return 27,
            (Some([1, 3]), [0, 4]) => return 28,
            (Some([1, 3]), [1, 2]) => return 29,
            (Some([1, 3]), [1, 4]) => return 30,
            (Some([1, 3]), [2, 2]) => return 31,
            (Some([1, 3]), [2, 3]) => return 32,
            (Some([1, 3]), [2, 4]) => return 33,
            (Some([1, 4]), [0, 4]) => return 34,
            (Some([1, 4]), [1, 3]) => return 35,
            (Some([1, 4]), [2, 4]) => return 36,
            (Some([2, 0]), [1, 0]) => return 37,
            (Some([2, 0]), [1, 1]) => return 38,
            (Some([2, 0]), [2, 1]) => return 39,
            (Some([2, 0]), [3, 0]) => return 40,
            (Some([2, 0]), [3, 1]) => return 41,
            (Some([2, 1]), [1, 1]) => return 42,
            (Some([2, 1]), [2, 0]) => return 43,
            (Some([2, 1]), [2, 2]) => return 44,
            (Some([2, 1]), [3, 1]) => return 45,
            (Some([2, 2]), [1, 1]) => return 46,
            (Some([2, 2]), [1, 2]) => return 47,
            (Some([2, 2]), [1, 3]) => return 48,
            (Some([2, 2]), [2, 1]) => return 49,
            (Some([2, 2]), [2, 3]) => return 50,
            (Some([2, 2]), [3, 1]) => return 51,
            (Some([2, 2]), [3, 2]) => return 52,
            (Some([2, 2]), [3, 3]) => return 53,
            (Some([2, 3]), [1, 3]) => return 54,
            (Some([2, 3]), [2, 2]) => return 55,
            (Some([2, 3]), [2, 4]) => return 56,
            (Some([2, 3]), [3, 3]) => return 57,
            (Some([2, 4]), [1, 3]) => return 58,
            (Some([2, 4]), [1, 4]) => return 59,
            (Some([2, 4]), [2, 3]) => return 60,
            (Some([2, 4]), [3, 3]) => return 61,
            (Some([2, 4]), [3, 4]) => return 62,
            (Some([3, 0]), [2, 0]) => return 63,
            (Some([3, 0]), [3, 1]) => return 64,
            (Some([3, 0]), [4, 0]) => return 65,
            (Some([3, 1]), [2, 0]) => return 66,
            (Some([3, 1]), [2, 1]) => return 67,
            (Some([3, 1]), [2, 2]) => return 68,
            (Some([3, 1]), [3, 0]) => return 69,
            (Some([3, 1]), [3, 2]) => return 70,
            (Some([3, 1]), [4, 0]) => return 71,
            (Some([3, 1]), [4, 1]) => return 72,
            (Some([3, 1]), [4, 2]) => return 73,
            (Some([3, 2]), [2, 2]) => return 74,
            (Some([3, 2]), [3, 1]) => return 75,
            (Some([3, 2]), [3, 3]) => return 76,
            (Some([3, 2]), [4, 2]) => return 77,
            (Some([3, 3]), [2, 2]) => return 78,
            (Some([3, 3]), [2, 3]) => return 79,
            (Some([3, 3]), [2, 4]) => return 80,
            (Some([3, 3]), [3, 2]) => return 81,
            (Some([3, 3]), [3, 4]) => return 82,
            (Some([3, 3]), [4, 2]) => return 83,
            (Some([3, 3]), [4, 3]) => return 84,
            (Some([3, 3]), [4, 4]) => return 85,
            (Some([3, 4]), [2, 4]) => return 86,
            (Some([3, 4]), [3, 3]) => return 87,
            (Some([3, 4]), [4, 4]) => return 88,
            (Some([4, 0]), [3, 0]) => return 89,
            (Some([4, 0]), [3, 1]) => return 90,
            (Some([4, 0]), [4, 1]) => return 91,
            (Some([4, 1]), [3, 1]) => return 92,
            (Some([4, 1]), [4, 0]) => return 93,
            (Some([4, 1]), [4, 2]) => return 94,
            (Some([4, 2]), [3, 1]) => return 95,
            (Some([4, 2]), [3, 2]) => return 96,
            (Some([4, 2]), [3, 3]) => return 97,
            (Some([4, 2]), [4, 1]) => return 98,
            (Some([4, 2]), [4, 3]) => return 99,
            (Some([4, 3]), [3, 3]) => return 100,
            (Some([4, 3]), [4, 2]) => return 101,
            (Some([4, 3]), [4, 4]) => return 102,
            (Some([4, 4]), [3, 3]) => return 103,
            (Some([4, 4]), [3, 4]) => return 104,
            (Some([4, 4]), [4, 3]) => return 105,
            (Some([0, 0]), [0, 2]) => return 106,
            (Some([0, 0]), [1, 0]) => return 107,
            (Some([0, 0]), [1, 1]) => return 108,
            (Some([0, 2]), [0, 0]) => return 109,
            (Some([1, 0]), [0, 0]) => return 110,
            (Some([1, 1]), [0, 0]) => return 111,
            (Some([0, 0]), [0, 1]) => return 112,
            (Some([0, 1]), [0, 0]) => return 113,
            (Some([0, 1]), [0, 3]) => return 114,
            (Some([0, 3]), [0, 1]) => return 115,
            (Some([0, 2]), [0, 4]) => return 116,
            (Some([0, 4]), [0, 2]) => return 117,
            (Some([0, 0]), [2, 0]) => return 118,
            (Some([2, 0]), [0, 0]) => return 119,
            (Some([0, 0]), [2, 2]) => return 120,
            (Some([0, 1]), [2, 1]) => return 121,
            (Some([0, 2]), [2, 0]) => return 122,
            (Some([1, 0]), [1, 2]) => return 123,
            (Some([1, 2]), [1, 0]) => return 124,
            (Some([2, 0]), [0, 2]) => return 125,
            (Some([2, 1]), [0, 1]) => return 126,
            (Some([2, 2]), [0, 0]) => return 127,
            (Some([0, 2]), [2, 2]) => return 128,
            (Some([1, 1]), [1, 3]) => return 129,
            (Some([1, 3]), [1, 1]) => return 130,
            (Some([2, 2]), [0, 2]) => return 131,
            (Some([0, 2]), [2, 4]) => return 132,
            (Some([0, 3]), [2, 3]) => return 133,
            (Some([0, 4]), [2, 2]) => return 134,
            (Some([1, 2]), [1, 4]) => return 135,
            (Some([1, 4]), [1, 2]) => return 136,
            (Some([2, 2]), [0, 4]) => return 137,
            (Some([2, 3]), [0, 3]) => return 138,
            (Some([2, 4]), [0, 2]) => return 139,
            (Some([0, 4]), [2, 4]) => return 140,
            (Some([2, 4]), [0, 4]) => return 141,
            (Some([1, 0]), [3, 0]) => return 142,
            (Some([3, 0]), [1, 0]) => return 143,
            (Some([1, 1]), [3, 1]) => return 144,
            (Some([2, 0]), [2, 2]) => return 145,
            (Some([2, 2]), [2, 0]) => return 146,
            (Some([3, 1]), [1, 1]) => return 147,
            (Some([1, 1]), [3, 3]) => return 148,
            (Some([1, 2]), [3, 2]) => return 149,
            (Some([1, 3]), [3, 1]) => return 150,
            (Some([2, 1]), [2, 3]) => return 151,
            (Some([2, 3]), [2, 1]) => return 152,
            (Some([3, 1]), [1, 3]) => return 153,
            (Some([3, 2]), [1, 2]) => return 154,
            (Some([3, 3]), [1, 1]) => return 155,
            (Some([1, 3]), [3, 3]) => return 156,
            (Some([2, 2]), [2, 4]) => return 157,
            (Some([2, 4]), [2, 2]) => return 158,
            (Some([3, 3]), [1, 3]) => return 159,
            (Some([1, 4]), [3, 4]) => return 160,
            (Some([3, 4]), [1, 4]) => return 161,
            (Some([2, 0]), [4, 0]) => return 162,
            (Some([4, 0]), [2, 0]) => return 163,
            (Some([2, 0]), [4, 2]) => return 164,
            (Some([2, 1]), [4, 1]) => return 165,
            (Some([2, 2]), [4, 0]) => return 166,
            (Some([3, 0]), [3, 2]) => return 167,
            (Some([3, 2]), [3, 0]) => return 168,
            (Some([4, 0]), [2, 2]) => return 169,
            (Some([4, 1]), [2, 1]) => return 170,
            (Some([4, 2]), [2, 0]) => return 171,
            (Some([2, 2]), [4, 2]) => return 172,
            (Some([3, 1]), [3, 3]) => return 173,
            (Some([3, 3]), [3, 1]) => return 174,
            (Some([4, 2]), [2, 2]) => return 175,
            (Some([2, 2]), [4, 4]) => return 176,
            (Some([2, 3]), [4, 3]) => return 177,
            (Some([2, 4]), [4, 2]) => return 178,
            (Some([3, 2]), [3, 4]) => return 179,
            (Some([3, 4]), [3, 2]) => return 180,
            (Some([4, 2]), [2, 4]) => return 181,
            (Some([4, 3]), [2, 3]) => return 182,
            (Some([4, 4]), [2, 2]) => return 183,
            (Some([2, 4]), [4, 4]) => return 184,
            (Some([4, 4]), [2, 4]) => return 185,
            (Some([4, 0]), [4, 2]) => return 186,
            (Some([4, 2]), [4, 0]) => return 187,
            (Some([4, 1]), [4, 3]) => return 188,
            (Some([4, 3]), [4, 1]) => return 189,
            (Some([4, 2]), [4, 4]) => return 190,
            (Some([4, 4]), [4, 2]) => return 191,
            _ => panic!("Invalid value to `m2i_tiger`"),
        }
    }
}
