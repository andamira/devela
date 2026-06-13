// devela/src/sys/os/term/grid/tests.rs

use crate::{TermColor, TermColors, TermGrid, TermGridError, TermPen, TermStyle, Termel};
use crate::{ext, pos, region};

#[test]
fn construction_and_geometry() {
    let storage = [0u8; 16];
    let grid = TermGrid::new(storage, ext!(3usize, 4usize)).unwrap();
    assert_eq!(grid.extent(), ext!(3, 4));
    assert_eq!(grid.width(), 3);
    assert_eq!(grid.height(), 4);
    assert_eq!(grid.len(), 12);
    assert_eq!(grid.spare_len(), 4);
    assert_eq!(grid.as_slice(), &[0; 12]);
}
#[test]
fn insufficient_storage() {
    let error = TermGrid::new([0u8; 5], ext!(3usize, 2usize)).unwrap_err();
    assert_eq!(error, TermGridError::NotEnoughElements { required: 6, available: 5 },);
}
#[test]
fn coordinate_mapping() {
    let grid = TermGrid::new([0u8; 12], ext!(4usize, 3usize)).unwrap();
    assert_eq!(grid.index_of(pos!(0usize, 0usize)), Some(0));
    assert_eq!(grid.index_of(pos!(3usize, 2usize)), Some(11));
    assert_eq!(grid.index_of(pos!(4usize, 2usize)), None);
    assert_eq!(grid.position_of(0), Some(pos!(0usize, 0usize)));
    assert_eq!(grid.position_of(11), Some(pos!(3usize, 2usize)));
    assert_eq!(grid.position_of(12), None);
}
#[test]
fn access_and_rows() {
    let mut grid = TermGrid::new([0, 1, 2, 3, 4, 5], ext!(3usize, 2usize)).unwrap();
    assert_eq!(grid.get_xy(1, 0), Some(&1));
    assert_eq!(grid.get_xy(2, 1), Some(&5));
    assert_eq!(grid.get_xy(3, 0), None);
    assert_eq!(grid.row(0), Some(&[0, 1, 2][..]));
    assert_eq!(grid.row(1), Some(&[3, 4, 5][..]));
    assert_eq!(grid.row(2), None);
    assert!(grid.set_xy(1, 1, 9));
    assert!(!grid.set_xy(3, 1, 9));
    assert_eq!(grid.row(1), Some(&[3, 9, 5][..]));
}
mod draw {
    use super::*;

    #[test]
    fn fill_only_affects_active_grid() {
        let mut grid = TermGrid::new([1u8; 8], ext!(3usize, 2usize)).unwrap();
        grid.fill(7);
        assert_eq!(grid.as_slice(), &[7; 6]);
        assert_eq!(grid.storage_slice(), &[7, 7, 7, 7, 7, 7, 1, 1]);
    }
    #[test]
    fn fill_region_clips() {
        let mut grid = TermGrid::new([0u8; 20], ext!(5, 4)).unwrap();
        grid.fill_region(region!(3, 2, 4, 4), 7);
        assert_eq!(grid.row(0).unwrap(), &[0, 0, 0, 0, 0]);
        assert_eq!(grid.row(1).unwrap(), &[0, 0, 0, 0, 0]);
        assert_eq!(grid.row(2).unwrap(), &[0, 0, 0, 7, 7]);
        assert_eq!(grid.row(3).unwrap(), &[0, 0, 0, 7, 7]);
    }
    #[test]
    fn lines_clip() {
        let mut grid = TermGrid::new([0u8; 15], ext!(5, 3)).unwrap();
        grid.hline(3, 0, 8, 1);
        grid.vline(1, 1, 8, 2);
        assert_eq!(grid.row(0).unwrap(), &[0, 0, 0, 1, 1]);
        assert_eq!(grid.row(1).unwrap(), &[0, 2, 0, 0, 0]);
        assert_eq!(grid.row(2).unwrap(), &[0, 2, 0, 0, 0]);
    }
    #[test]
    fn frame_handles_thin_regions() {
        let mut grid = TermGrid::new([0u8; 15], ext!(5, 3)).unwrap();
        grid.frame(region!(1, 0, 3, 3), 1);
        assert_eq!(grid.row(0).unwrap(), &[0, 1, 1, 1, 0]);
        assert_eq!(grid.row(1).unwrap(), &[0, 1, 0, 1, 0]);
        assert_eq!(grid.row(2).unwrap(), &[0, 1, 1, 1, 0]);
    }
    #[test]
    fn blit_at_clips_destination() {
        let source = TermGrid::new([1u8, 2, 3, 4, 5, 6], ext!(3, 2)).unwrap();
        let mut destination = TermGrid::new([0u8; 12], ext!(4, 3)).unwrap();
        destination.blit_at(&source, pos!(2, 1));
        assert_eq!(destination.row(0).unwrap(), &[0, 0, 0, 0]);
        assert_eq!(destination.row(1).unwrap(), &[0, 0, 1, 2]);
        assert_eq!(destination.row(2).unwrap(), &[0, 0, 4, 5]);
    }
    #[test]
    fn blit_region_clips_source() {
        let source = TermGrid::new([1u8, 2, 3, 4, 5, 6, 7, 8, 9], ext!(3, 3)).unwrap();
        let mut destination = TermGrid::new([0u8; 6], ext!(3, 2)).unwrap();
        destination.blit_region_at(&source, region!(1, 1, 8, 8), pos!(0, 0));
        assert_eq!(destination.row(0).unwrap(), &[5, 6, 0]);
        assert_eq!(destination.row(1).unwrap(), &[8, 9, 0]);
    }
    #[test]
    fn pen_constructs_termels() {
        let colors = TermColors::new(TermColor::indexed(2), TermColor::indexed(0));
        let pen = TermPen::new(TermStyle::BOLD, colors);
        let termel = pen.termel('●');
        assert_eq!(*termel.textel().value(), '●');
        assert_eq!(*termel.style(), TermStyle::BOLD);
        assert_eq!(*termel.colors(), colors);
    }
}
mod text {
    use super::*;

    #[test]
    fn write_str_clips_right() {
        let blank = Termel::plain_const(' ');
        let mut grid = TermGrid::new([blank; 10], ext!(5, 2)).unwrap();
        let written = grid.write_str_xy(3, 0, "abcd", TermPen::PLAIN);
        assert_eq!(written, 2);
        let row = grid.row(0).unwrap();
        assert!(row.iter().map(|cell| *cell.textel().value()).eq([' ', ' ', ' ', 'a', 'b']),);
    }
    #[test]
    fn write_str_outside_writes_nothing() {
        let blank = Termel::plain_const(' ');
        let mut grid = TermGrid::new([blank; 5], ext!(5, 1)).unwrap();
        assert_eq!(grid.write_str_xy(5, 0, "abc", TermPen::PLAIN), 0);
        assert_eq!(grid.write_str_xy(0, 1, "abc", TermPen::PLAIN), 0);
    }
    #[test]
    fn write_str_returns_scalar_cell_count() {
        let blank = Termel::plain_const(' ');
        let mut grid = TermGrid::new([blank; 5], ext!(5, 1)).unwrap();
        let written = grid.write_str_xy(0, 0, "Aλ🦀", TermPen::PLAIN);
        assert_eq!(written, 3);
        assert_eq!(*grid.get_xy(0, 0).unwrap().textel().value(), 'A');
        assert_eq!(*grid.get_xy(1, 0).unwrap().textel().value(), 'λ');
        assert_eq!(*grid.get_xy(2, 0).unwrap().textel().value(), '🦀');
    }
    #[test]
    fn write_str_stops_at_line_break() {
        let blank = Termel::plain_const(' ');
        let mut grid = TermGrid::new([blank; 5], ext!(5, 1)).unwrap();
        let written = grid.write_str_xy(0, 0, "ab\ncd", TermPen::PLAIN);
        assert_eq!(written, 2);
        assert_eq!(*grid.get_xy(0, 0).unwrap().textel().value(), 'a');
        assert_eq!(*grid.get_xy(1, 0).unwrap().textel().value(), 'b');
        assert_eq!(*grid.get_xy(2, 0).unwrap().textel().value(), ' ');
    }
}
