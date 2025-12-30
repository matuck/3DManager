/*
 * Copyright (c) 2025-2026 Mitch Tuck
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use iced::Length;
use iced::widget::{button, text, Container, row, column};
use crate::{Message, ThreeDPrintManager};

impl ThreeDPrintManager {
    pub fn project(&self) -> Container<'_, Message> {
        let mut proj = self.selected_project.clone().unwrap();
        let main_content = iced::widget::column![]
            .push(
                row![
                        column![text(proj.name.to_string()).size(50)].width(Length::Fill),
                        column![
                            button("Open Directory").on_press(Message::OpenDirectory(proj.path)),
                            button("Back").on_press(Message::ToMainPage)
                        ]
                ].width(Length::Fill)
            );
        Container::new(main_content).width(Length::Fill).height(Length::Fill)
    }
}