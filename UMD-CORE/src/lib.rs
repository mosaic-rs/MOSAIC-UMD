/*
This file is part of MOSAIC.

MOSAIC-UMD is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status() {
        assert_eq!(status::status(), "MOSAIC-UMD called");
    }
}
