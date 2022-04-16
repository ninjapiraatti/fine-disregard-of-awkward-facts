pub mod rng;
pub mod program;
pub mod utils;

lazy_static::lazy_static! {
	static ref CHARS_ALL: String = String::from("ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓█│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂");
	static ref CHARS_HRZ: String = String::from("⌐¬½¼┴┬─═╧╨╤±≥≤↔-=");
	static ref CHARS_VRT: String = String::from("│┤╡╢╣║┼╞╟╠╬█╫╪⌡|");
	static ref CHARS_TOPLEFT: String = String::from("╔╥╒┌");
	static ref CHARS_TOPRIGHT: String = String::from("╖╕╗┐");
	static ref CHARS_BOTTOMLEFT: String = String::from("╙╚╧└");
	static ref CHARS_BOTTOMRIGHT: String = String::from("╝╜╛┘");
	static ref CHARS_TOPHRZ: String = String::from("▓█▀");
	static ref CHARS_BOTTOMHRZ: String = String::from("▓█_▄");
	static ref CHARS_FILL: String = String::from("░▒▓█");
	static ref CHARS_RND: String = String::from("ÇêëèôöÖóúªº¿ßσµΦΘΩδ∞°²☺☻♥♦♣♠•○♂♀☼%0@");
	static ref SIZE: (u16, u16) = termion::terminal_size().unwrap();
}
