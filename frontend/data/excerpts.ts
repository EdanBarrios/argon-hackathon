// Verbatim excerpts of the two parse_string implementations the recorded run
// compared. C: cJSON v1.7.19 cJSON.c (MIT, DaveGamble/cJSON). Rust: the draft
// Gemini 3.8 Flash wrote today, results/cjson-flash-vs-argon/flash-2/cjson.rs.
// `hl` marks the line where the two disagree on "\uZ\t\\".

export interface CodeLine {
  n: number | null;
  text: string;
  hl?: boolean;
}

export const C_PARSE_STRING: CodeLine[] = [
  { n: 819, text: "static cJSON_bool parse_string(cJSON * const item, parse_buffer * const input_buffer)" },
  { n: 820, text: "{" },
  { n: null, text: "    /* … pass one: find the closing quote, pairing escapes … */" },
  { n: 865, text: "    output_pointer = output;" },
  { n: 866, text: "    /* loop through the string literal */" },
  { n: 867, text: "    while (input_pointer < input_end)" },
  { n: 868, text: "    {" },
  { n: 869, text: "        if (*input_pointer != '\\\\')" },
  { n: 870, text: "        {" },
  { n: 871, text: "            *output_pointer++ = *input_pointer++;" },
  { n: 872, text: "        }" },
  { n: 873, text: "        /* escape sequence */" },
  { n: 874, text: "        else" },
  { n: 875, text: "        {" },
  { n: 876, text: "            unsigned char sequence_length = 2;" },
  { n: 877, text: "            if ((input_end - input_pointer) < 1)", hl: true },
  { n: 878, text: "            {" },
  { n: 879, text: "                goto fail;" },
  { n: 880, text: "            }" },
  { n: 881, text: "" },
  { n: 882, text: "            switch (input_pointer[1])" },
  { n: 883, text: "            {" },
  { n: 884, text: "                case 'b':" },
  { n: 885, text: "                    *output_pointer++ = '\\b';" },
  { n: 886, text: "                    break;" },
  { n: 887, text: "                case 'f':" },
  { n: 888, text: "                    *output_pointer++ = '\\f';" },
  { n: 889, text: "                    break;" },
];

export const RUST_PARSE_STRING: CodeLine[] = [
  { n: 138, text: "fn parse_string(buffer: &mut ParseBuffer) -> Result<Vec<u8>, ()> {" },
  { n: null, text: "    // … pass one: find the closing quote, pairing escapes …" },
  { n: 165, text: "    let mut input_pointer = start_offset + 1;" },
  { n: 166, text: "    let capacity = (input_end - start_offset).saturating_sub(skipped_bytes);" },
  { n: 167, text: "    let mut output = Vec::with_capacity(capacity);" },
  { n: 168, text: "" },
  { n: 169, text: "    while input_pointer < input_end {" },
  { n: 170, text: "        let b = buffer.content[input_pointer];" },
  { n: 171, text: "        if b != b'\\\\' {" },
  { n: 172, text: "            output.push(b);" },
  { n: 173, text: "            input_pointer += 1;" },
  { n: 174, text: "        } else {" },
  { n: 175, text: "            if input_end - input_pointer < 2 {", hl: true },
  { n: 176, text: "                buffer.offset = input_pointer;" },
  { n: 177, text: "                return Err(());" },
  { n: 178, text: "            }" },
  { n: 179, text: "            let escape = buffer.content[input_pointer + 1];" },
  { n: 180, text: "            match escape {" },
  { n: 181, text: "                b'b' => {" },
  { n: 182, text: "                    output.push(0x08);" },
  { n: 183, text: "                    input_pointer += 2;" },
  { n: 184, text: "                }" },
  { n: 185, text: "                b'f' => {" },
  { n: 186, text: "                    output.push(0x0C);" },
  { n: 187, text: "                    input_pointer += 2;" },
  { n: 188, text: "                }" },
];
