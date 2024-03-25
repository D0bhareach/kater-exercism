#include "isogram.h"
#include <stdlib.h>
#include <ctype.h>

bool is_isogram(const char phrase[])
{

    if (phrase == NULL)
    {
        return false;
    }

    size_t len = 0;
    while (phrase[len] != '\0') len++;

    if (len == 0)
    {
        return true;
    }

    typedef struct 
    {
        size_t index;
        char * chars;
    } Isogram;
    Isogram iso;
    iso.index = 0;
    iso.chars = malloc(len *  sizeof(char));
    for (size_t i = 0; i < len; i++)
    {
            char phrase_char = phrase[i];
            phrase_char = tolower(phrase_char);
            bool has_char = false;
            if (i == 0 || ispunct(phrase_char) || isspace(phrase_char))
            {
                iso.chars[iso.index] = phrase_char;
                iso.index++;
                continue;
            }
        for (size_t j = 0; j < iso.index ; j++)
        {
            if (iso.chars[j] == phrase_char)
            {
                has_char = true;
                break;
            }
        }
        if(!has_char)
        {
            iso.chars[iso.index] = phrase_char;
            iso.index++;
        }
    }

    free(iso.chars);
    return len == iso.index;
}
