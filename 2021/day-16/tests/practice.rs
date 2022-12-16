#[test]
fn part_1() {
    assert_eq!(day_16::part_1("8A004A801A8002F478"), 16);
    assert_eq!(day_16::part_1("620080001611562C8802118E34"), 12);
    assert_eq!(day_16::part_1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(day_16::part_1("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn part_2() {
    assert_eq!(day_16::part_2("C200B40A82"), 3);
    assert_eq!(day_16::part_2("04005AC33890"), 54);
    assert_eq!(day_16::part_2("880086C3E88112"), 7);
    assert_eq!(day_16::part_2("CE00C43D881120"), 9);
    assert_eq!(day_16::part_2("D8005AC2A8F0"), 1);
    assert_eq!(day_16::part_2("F600BC2D8F"), 0);
    assert_eq!(day_16::part_2("9C005AC2F8F0"), 0);
    assert_eq!(day_16::part_2("9C0141080250320F1802104A08"), 1);
}
