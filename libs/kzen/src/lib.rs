/*
 * This file is part of the Zen distribution (https://github.com/fxttr/zen).
 * Copyright (c) 2023 Florian Marrero Liestmann.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

pub fn wrmsr(msr: u32, value: u64) {
    unsafe {
	x86_64::registers::model_specific::Msr::new(msr).write(value)
    }
}

pub fn rdmsr(msr: u32) -> u64 {
    unsafe {
	x86_64::registers::model_specific::Msr::new(msr).read()
    }
}