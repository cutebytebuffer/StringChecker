pub struct Signature {
    pub name: &'static str,
    pub strings: &'static [&'static str],
}

pub const CHEATS: &[Signature] = &[
    Signature {
        name: "Combat Modules",
        strings: &[
            "killaura", "anchoraura", "crystalaura", "aimassist", "backtrack", "fakelag" ,
            "pingspoof", "hitboxes", "maceaura", "reach", "triggerbot", "targetstrafe"
        ],
    },

    Signature {
        name: "Macro",
        strings: &[
            "anchormacro", "crystalmacro", "autohitcrystal", "autoanchor"
        ],
    },


    Signature {
        name: "Movement module",
        strings: &[
            "antivoid", "fly", "flight", "longjump", "superjump"
        ],
    },



];