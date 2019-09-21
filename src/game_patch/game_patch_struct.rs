/*
 * SlashGaming Diablo II Modding API - Rust FFI
 * Copyright (C) 2019  Mir Drualga
 *
 * This file is part of SlashGaming Diablo II Modding API - Rust FFI.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Additional permissions under GNU Affero General Public License version 3
 * section 7
 *
 * If you modify this Program, or any covered work, by linking or combining
 * it with Diablo II (or a modified version of that game and its
 * libraries), containing parts covered by the terms of Blizzard End User
 * License Agreement, the licensors of this Program grant you additional
 * permission to convey the resulting work. This additional permission is
 * also extended to any combination of expansions, mods, and remasters of
 * the game.
 *
 * If you modify this Program, or any covered work, by linking or combining
 * it with any Graphics Device Interface (GDI), DirectDraw, Direct3D,
 * Glide, OpenGL, or Rave wrapper (or modified versions of those
 * libraries), containing parts not covered by a compatible license, the
 * licensors of this Program grant you additional permission to convey the
 * resulting work.
 *
 * If you modify this Program, or any covered work, by linking or combining
 * it with any library (or a modified version of that library) that links
 * to Diablo II (or a modified version of that game and its libraries),
 * containing parts not covered by a compatible license, the licensors of
 * this Program grant you additional permission to convey the resulting
 * work.
 */

extern crate libc;

use crate::game_address::game_address_struct;

#[repr(C)]
pub struct MAPI_GamePatch {
  game_address: game_address_struct::MAPI_GameAddress,
  is_patch_applied: bool,
  patch_buffer: *mut u8,
  old_buffer: *mut u8,
  patch_size: libc::size_t
}

extern {
  /**
   * Deinitializes the specified game patch.
   */
  pub fn MAPI_GamePatch_Deinit(
      game_patch: *mut MAPI_GamePatch
  );

  /**
   * Applies the game patch by replacing the bytes at its target address with
   * the bytes stored in its buffer.
   */
  pub fn MAPI_GamePatch_Apply(
      game_patch: *mut MAPI_GamePatch
  );

  /**
   * Removes the effects of the game patch by restoring the original state of
   * the bytes at its target address.
   */
  pub fn MAPI_GamePatch_Remove(
      game_patch: *mut MAPI_GamePatch
  );
}
