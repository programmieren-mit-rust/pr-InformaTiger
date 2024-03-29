// Here all of the files of the package tests have to be added.
// If they are added, they get executed when cargo run is called.
#[cfg(test)]
mod escape_tests;

#[cfg(test)]
mod file_handler_tests;

#[cfg(test)]
mod histogram_tests;

#[cfg(test)]
mod picture_tests;

#[cfg(test)]
mod search_index_tests;

#[cfg(test)]
mod with_threads_tests;

#[cfg(test)]
mod average_brightness_tests;

#[cfg(test)]
mod lib_tests;
