#include "grade_school.h"
#include <stdlib.h>
#include <string.h>
//
#include <stdio.h>
/*
 *
 typedef struct {
 size_t count;
 student_t students[MAX_STUDENTS];
 } roster_t;


 typedef struct {
 uint8_t grade;
 char name[MAX_NAME_LENGTH];
 } student_t;
 * */
// My functions
int compare_grade(const void *a, const void *b);
int compare_name(const void *a, const void *b);
int roster_has_student(roster_t *roster, char * name);

int compare_grade(const void *a, const void *b)
{
        uint8_t grade1 = ((const student_t *)a)->grade;
        uint8_t grade2 = ((const  student_t *)b)->grade;
        if (grade2 == 255) return -1;
        // printf("1grade: %d, 2grade: %d\n", grade1, grade2);
        return (grade1 - grade2);
}

int compare_name(const void *a, const void *b)
{
        const char *n1 = ((const student_t *)a)->name;
        const char *n2 = ((const student_t *)b)->name;
        // const uint8_t g = ((const student_t *)b)->grade;
        // if (*n2 == '\0' || g == 255) return -1;
        // printf("%.*s",len,buf);
        // printf("compare_name- 1name: %.*s, 2name: %.*s\n", 20, n1, 20, n2);
        return strcmp(n1, n2);
}

/*
 *If roster has student will return int > 0;
 * */
int roster_has_student(roster_t *roster, char * name)
{
        // if roster students have the same name they can not be in the roster
        // student is unique by name.
        int res = 0;
        for (int i = 0; i  < MAX_STUDENTS; i++){
                student_t s = roster->students[i];
                int cmp = strcmp(s.name, name);
                if (cmp == 0){
                        // printf("grade is different, but names are the same.\n");
                        // printf("s.name: %s, name: %s\n", s.name, name);
                        return 1;
                }

        }
        return res;
}

void init_roster(roster_t *rost)
{
        rost->count = 0;
        // create dummy student
        student_t s;
        s.grade = 255;
        for (int i = 0; i <  MAX_NAME_LENGTH; i++){
                s.name[i] = '\0';
        }
        // add dummy student to students
        for (int i = 0; i  < MAX_STUDENTS; i++){
                rost->students[i] = s;
        }

}


uint8_t add_student(roster_t *rost, char * name, uint8_t grade)
{
        uint8_t res = 0;
        if (!roster_has_student(rost, name)) {
                student_t st;
                st.grade = grade;
                strcpy(st.name, name);
                rost->students[rost->count] = st;
                rost->count++;
                res++;
                qsort(rost->students, MAX_STUDENTS,
                                sizeof(student_t), compare_name);
                qsort(rost->students, MAX_STUDENTS,
                                sizeof(student_t), compare_grade);
        }
        return res;
}

roster_t get_grade(roster_t *rost, uint8_t desired)
{
        roster_t r;
        r.count = 0;
        // loop and add if grade is eq
        for (int i = 0; i  < MAX_STUDENTS; i++){
                student_t s = rost->students[i];
                if (s.grade == desired) {
                        r.students[r.count] = s;
                        r.count++;
                }

        }

        // printf("{\n");
        // printf("count: %ld\n", r.count);
        // for (int i = 0; i  < MAX_STUDENTS; i++){
        //         student_t s = r.students[i];
        //         if (s.grade !=255 && s.name[0] != '\0')
        //                 printf("name: %s, grade: %d\n", s.name, s.grade);
        // }
        // printf("}\n");
        return r;
}
