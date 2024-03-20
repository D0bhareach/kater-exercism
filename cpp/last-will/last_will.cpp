// Enter your code below the lines of the families' information

// Secret knowledge of the Zhang family:
namespace zhang {
    int bank_number_part(int secret_modifier) {
        int zhang_part{8'541};
        return (zhang_part*secret_modifier) % 10000;
    }
    namespace red {
        constexpr int code_fragment() {return 512;}
    }
    namespace blue {
        constexpr int code_fragment() {return 677;}
    }
}

// Secret knowledge of the Khan family:
namespace khan {
    int bank_number_part(int secret_modifier) {
        int khan_part{4'142};
        return (khan_part*secret_modifier) % 10000;
    }
    namespace red {
        constexpr int code_fragment() {return 148;}
    }
    namespace blue {
        constexpr int code_fragment() {return 875;}
    }
}

// Secret knowledge of the Garcia family:
namespace garcia {
    int bank_number_part(int secret_modifier) {
        int garcia_part{4'023};
        return (garcia_part*secret_modifier) % 10000;
    }
    namespace red {
        constexpr int code_fragment() {return 118;}
    }
    namespace blue {
        constexpr int code_fragment() {return 923;}
    }
}

// Enter your code below
namespace estate_executor {
    // aliases to make things interesting
    auto& z_bn = zhang::bank_number_part;
    auto& zr_cf = zhang::red::code_fragment;
    auto& zb_cf = zhang::blue::code_fragment;

    auto& k_bn = khan::bank_number_part;
    auto& kr_cf = khan::red::code_fragment;
    auto& kb_cf= khan::blue::code_fragment;

    auto& g_bn = garcia::bank_number_part;
    auto& gr_cf = garcia::red::code_fragment;
    auto& gb_cf = garcia::blue::code_fragment;

int assemble_account_number(int secret_modifier){
    return
        z_bn(secret_modifier) +
        k_bn(secret_modifier) +
        g_bn(secret_modifier);}

int assemble_code(){
    return
        (zr_cf() + kr_cf() + gr_cf()) * 
        (zb_cf() + kb_cf() + gb_cf()); 
    }

}