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
mod suchindex_tests;
mod test_averagebrightness;

#[cfg(test)]
mod with_threads_tests;

#[cfg(test)]
mod test_2a;

#[cfg(test)]
mod lib_tests;
