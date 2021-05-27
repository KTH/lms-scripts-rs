# List enrollments

This script creates a CSV file with students and teachers enrolled in a Canvas examroom/courseroom

## Run it!

Prepare a `.env` file with the required environmental variables as written in `.env.in`. We recommend that you place that `.env` file in the parent directory so you can use the same variables across the whole repository.

## Notes

- When choosing "examroom", the script will prompt for a range of dates. Meaning that you will get enrollments for examrooms that are linked with exams that happen within such range
- When choosing "courseroom", the script will prompt for year-period. You will get enrollments for courserooms linked with course rounds in that period.

The script will always read enrollments directly from Canvas, will not use any other source (like UG, Ladok, etc)
