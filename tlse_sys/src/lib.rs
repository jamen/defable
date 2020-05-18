#![feature(abi_thiscall)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[path = "n_game_text/mod.rs"]
pub mod NGameText;
#[path = "n_display_view/mod.rs"]
pub mod NDisplayView;
#[path = "n_entity_events/mod.rs"]
pub mod NEntityEvents;

pub mod cxx;

mod c_2d_box_f;
mod c_2d_coord_i;
mod c_2d_vector;
mod c_3d_animation_manager;
mod c_3d_vector;
mod c_ai_game_camera_base;
mod c_array;
mod c_atmos_copy_info;
mod c_atmos_processor;
mod c_bank_file_async;
mod c_bank_file_entry_update_data;
mod c_bank_file;
mod c_bank_state_block;
mod c_base_class_non_copyable;
mod c_base_class;
mod c_base_intelligent_pointer;
mod c_base_object_pointer;
mod c_base_object;
mod c_basic_string;
mod c_bullet_time_manager;
mod c_camera;
mod c_char_string;
mod c_combat_manager;
mod c_counted_pointer;
mod c_counter;
mod c_crc_symbol_map;
mod c_def_class_base;
mod c_def_pointer;
mod c_def_string;
mod c_definition_manager;
mod c_device_reset_callback;
mod c_disk_file_win32;
mod c_display_engine;
mod c_display_view_manager;
mod c_engine_primitive_handle;
mod c_environment;
mod c_event_package_file_header;
mod c_faction_manager;
mod c_fade_in_fade_out_base;
mod c_font_bank;
mod c_frame_rate_smoother;
mod c_game_component;
mod c_game_definition_manager;
mod c_game_event_dispatch;
mod c_game_event_package_set;
mod c_game_event_package;
mod c_game_event;
mod c_game_player_interface;
mod c_game_script_interface_base;
mod c_game_script_interface;
mod c_game_time_manager;
mod c_game;
mod c_generic_var;
mod c_graphic_data_bank;
mod c_hero_combat_def;
mod c_hero_log_book_entry;
mod c_hero_log_book;
mod c_init_base_class;
mod c_input_process_main;
mod c_input_process_manager;
mod c_intelligent_pointer;
mod c_interpolation_info;
mod c_key_pair_compare_less;
mod c_letter_box_mode_info;
mod c_linked_list;
mod c_lua;
mod c_main_game_component_init;
mod c_main_game_component;
mod c_mesh_data_bank;
mod c_message_event_manager;
mod c_music_manager;
mod c_navigation_manager;
mod c_network_client;
mod c_network_player;
mod c_network_server;
mod c_opinion_reaction_manager;
mod c_packed_uint_array;
mod c_parent_def_class_base;
mod c_player_def;
mod c_player_manager;
mod c_player_movement_def;
mod c_player;
mod c_processed_input;
mod c_rgb_colour;
mod c_rgb_float_colour;
mod c_right_handed_set;
mod c_rough_frame_counter;
mod c_screen_filter_s_thing_by_pass;
mod c_script_conversation_manager;
mod c_script_game_resource_object_base;
mod c_script_game_resource_object_movie_base;
mod c_script_game_resource_object_scripted_thing_base;
mod c_script_info_manager;
mod c_script_thing;
mod c_test_quest;
mod c_thing_creature_base;
mod c_thing_manager;
mod c_thing_player_creature;
mod c_thing_search_tools;
mod c_thing;
mod c_timer;
mod c_vector_map;
mod c_wide_string;
mod c_world_map;
mod c_world;
mod ca_game_camera_base;
mod ca_input_process;
mod ca_sound_bank;
mod ci_engine;
mod e_camera_op;
mod e_category;
mod e_clothing_covers_area;
mod e_creature_group;
mod e_crime;
mod e_cutscene_behavior;
mod e_door_trigger_type;
mod e_game_action;
mod e_hero_ability;
mod e_hero_title;
mod e_hero_trainable_stat_type;
mod e_mini_game_type;
mod e_morality;
mod e_music_set_type;
mod e_opinion_attitude_type;
mod e_opinion_deed_type;
mod e_opinion_post_deed_type;
mod e_opinion_reaction_type;
mod e_opinion;
mod e_script_ai_priority;
mod e_script_village_attitude;
mod e_scripting_state_groups;
mod e_sex;
mod e_targeting_type;
mod e_text_group_selection_method;
mod e_tutorial_category;
mod e_movement_band_type;
mod e_controlled_movemen_type;
mod e_player_mode;

pub use c_2d_box_f::*;
pub use c_2d_coord_i::*;
pub use c_2d_vector::*;
pub use c_3d_animation_manager::*;
pub use c_3d_vector::*;
pub use c_ai_game_camera_base::*;
pub use c_array::*;
pub use c_atmos_copy_info::*;
pub use c_atmos_processor::*;
pub use c_bank_file_async::*;
pub use c_bank_file_entry_update_data::*;
pub use c_bank_file::*;
pub use c_bank_state_block::*;
pub use c_base_class_non_copyable::*;
pub use c_base_class::*;
pub use c_base_intelligent_pointer::*;
pub use c_base_object_pointer::*;
pub use c_base_object::*;
pub use c_basic_string::*;
pub use c_bullet_time_manager::*;
pub use c_camera::*;
pub use c_char_string::*;
pub use c_combat_manager::*;
pub use c_counted_pointer::*;
pub use c_counter::*;
pub use c_crc_symbol_map::*;
pub use c_def_class_base::*;
pub use c_def_pointer::*;
pub use c_def_string::*;
pub use c_definition_manager::*;
pub use c_device_reset_callback::*;
pub use c_disk_file_win32::*;
pub use c_display_engine::*;
pub use c_display_view_manager::*;
pub use c_engine_primitive_handle::*;
pub use c_environment::*;
pub use c_event_package_file_header::*;
pub use c_faction_manager::*;
pub use c_fade_in_fade_out_base::*;
pub use c_font_bank::*;
pub use c_frame_rate_smoother::*;
pub use c_game_component::*;
pub use c_game_definition_manager::*;
pub use c_game_event_dispatch::*;
pub use c_game_event_package_set::*;
pub use c_game_event_package::*;
pub use c_game_event::*;
pub use c_game_player_interface::*;
pub use c_game_script_interface_base::*;
pub use c_game_script_interface::*;
pub use c_game_time_manager::*;
pub use c_game::*;
pub use c_generic_var::*;
pub use c_graphic_data_bank::*;
pub use c_hero_combat_def::*;
pub use c_hero_log_book_entry::*;
pub use c_hero_log_book::*;
pub use c_init_base_class::*;
pub use c_input_process_main::*;
pub use c_input_process_manager::*;
pub use c_intelligent_pointer::*;
pub use c_interpolation_info::*;
pub use c_key_pair_compare_less::*;
pub use c_letter_box_mode_info::*;
pub use c_linked_list::*;
pub use c_lua::*;
pub use c_main_game_component_init::*;
pub use c_main_game_component::*;
pub use c_mesh_data_bank::*;
pub use c_message_event_manager::*;
pub use c_music_manager::*;
pub use c_navigation_manager::*;
pub use c_network_client::*;
pub use c_network_player::*;
pub use c_network_server::*;
pub use c_opinion_reaction_manager::*;
pub use c_packed_uint_array::*;
pub use c_parent_def_class_base::*;
pub use c_player_def::*;
pub use c_player_manager::*;
pub use c_player_movement_def::*;
pub use c_player::*;
pub use c_processed_input::*;
pub use c_rgb_colour::*;
pub use c_rgb_float_colour::*;
pub use c_right_handed_set::*;
pub use c_rough_frame_counter::*;
pub use c_screen_filter_s_thing_by_pass::*;
pub use c_script_conversation_manager::*;
pub use c_script_game_resource_object_base::*;
pub use c_script_game_resource_object_movie_base::*;
pub use c_script_game_resource_object_scripted_thing_base::*;
pub use c_script_info_manager::*;
pub use c_script_thing::*;
pub use c_test_quest::*;
pub use c_thing_creature_base::*;
pub use c_thing_manager::*;
pub use c_thing_player_creature::*;
pub use c_thing_search_tools::*;
pub use c_thing::*;
pub use c_timer::*;
pub use c_vector_map::*;
pub use c_wide_string::*;
pub use c_world_map::*;
pub use c_world::*;
pub use ca_game_camera_base::*;
pub use ca_input_process::*;
pub use ca_sound_bank::*;
pub use ci_engine::*;
pub use e_camera_op::*;
pub use e_category::*;
pub use e_clothing_covers_area::*;
pub use e_creature_group::*;
pub use e_crime::*;
pub use e_cutscene_behavior::*;
pub use e_door_trigger_type::*;
pub use e_game_action::*;
pub use e_hero_ability::*;
pub use e_hero_title::*;
pub use e_hero_trainable_stat_type::*;
pub use e_mini_game_type::*;
pub use e_morality::*;
pub use e_music_set_type::*;
pub use e_opinion_attitude_type::*;
pub use e_opinion_deed_type::*;
pub use e_opinion_post_deed_type::*;
pub use e_opinion_reaction_type::*;
pub use e_opinion::*;
pub use e_script_ai_priority::*;
pub use e_script_village_attitude::*;
pub use e_scripting_state_groups::*;
pub use e_sex::*;
pub use e_targeting_type::*;
pub use e_text_group_selection_method::*;
pub use e_tutorial_category::*;
pub use e_movement_band_type::*;
pub use e_controlled_movemen_type::*;
pub use e_player_mode::*;

/// An unknown type with zero bytes.
#[derive(Debug)]
#[repr(C)]
pub struct UnknownEmptyType;