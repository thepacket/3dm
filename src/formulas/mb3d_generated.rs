    GeneratedFormula {
        name: "ABoxMod1",
        source: "ABoxMod1.m3f",
        param_floats: 8,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Fold", kind: ParamKind::Float, offset: 3, default: &[2.0] },
            GeneratedParam { path: "Scale vary", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "FoldXMod", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "FoldYMod", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "FoldZMod", kind: ParamKind::Float, offset: 7, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).actual_scale = select((*aux).actual_scale, __MB2P0__, ((*aux).i <= 0.0));
		(*aux).actual_scale = ((*aux).actual_scale + ((abs((*aux).actual_scale) - 1.0) * __MB2P4__));
		z.w = (z.w * select(select((*aux).actual_scale, ((*aux).actual_scale / (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__)))))), (1.0 >= (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))))), __MB2P1__, ((((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))) < __MB2P2__)));
		z.z = ((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * select(select((*aux).actual_scale, ((*aux).actual_scale / (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__)))))), (1.0 >= (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))))), __MB2P1__, ((((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))) < __MB2P2__))) + (*aux).const_c.z);
		z.y = ((((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * select(select((*aux).actual_scale, ((*aux).actual_scale / (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__)))))), (1.0 >= (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))))), __MB2P1__, ((((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))) < __MB2P2__))) + (*aux).const_c.x);
		z.x = ((((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * select(select((*aux).actual_scale, ((*aux).actual_scale / (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__)))))), (1.0 >= (((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))))), __MB2P1__, ((((((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__))) * ((-(abs((abs((z.y + __MB2P6__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P6__)))) + (((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))) * ((-(abs((abs((z.x + __MB2P5__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P5__))))) + (((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))) * ((-(abs((abs((z.z + __MB2P7__)) - __MB2P3__))) + __MB2P3__) + -(abs(__MB2P7__))))) < __MB2P2__))) + (*aux).const_c.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "ABoxMod2",
        source: "ABoxMod2.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.5] },
            GeneratedParam { path: "Fold XY", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Fold Z", kind: ParamKind::Float, offset: 4, default: &[1.5] },
            GeneratedParam { path: "Cyl HalfSize", kind: ParamKind::Float, offset: 5, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (z.w * select(select(__MB2P0__, (__MB2P0__ / select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__))), (1.0 >= select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)))), __MB2P1__, (select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)) < __MB2P2__)));
		z.z = (((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z)) * select(select(__MB2P0__, (__MB2P0__ / select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__))), (1.0 >= select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)))), __MB2P1__, (select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)) < __MB2P2__))) + (*aux).const_c.z);
		z.y = ((__MB2P4__ * select(select(__MB2P0__, (__MB2P0__ / select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__))), (1.0 >= select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)))), __MB2P1__, (select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)) < __MB2P2__))) + (*aux).const_c.y);
		z.x = (((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * select(select(__MB2P0__, (__MB2P0__ / select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__))), (1.0 >= select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)))), __MB2P1__, (select(((__MB2P4__ * __MB2P4__) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), ((((abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__) * (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) - __MB2P5__)) + (__MB2P4__ * __MB2P4__)) + ((abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)) * (abs((z.x + __MB2P3__)) - (abs((z.x - __MB2P3__)) + z.x)))), (abs((abs((z.z + __MB2P3__)) - (abs((z.z - __MB2P3__)) + z.z))) > __MB2P5__)) < __MB2P2__))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "ABoxVaryScale",
        source: "ABoxVaryScale.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.5] },
            GeneratedParam { path: "Fold", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Scale vary", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "R power", kind: ParamKind::Float, offset: 5, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).actual_scale = select((*aux).actual_scale, __MB2P0__, ((*aux).i <= 0.0));
		(*aux).actual_scale = ((*aux).actual_scale + ((abs((*aux).actual_scale) - 1.0) * __MB2P4__));
		z.w = (z.w * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__)));
		z.z = ((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.z);
		z.y = ((__MB2P3__ * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.y);
		z.x = (((__MB2P3__ + __MB2P3__) * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634) - round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634))) + round(((log((((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__)))))))) + (__MB2P3__ * __MB2P3__)) + ((__MB2P3__ + __MB2P3__) * (__MB2P3__ + __MB2P3__)))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "ABoxVaryScale2",
        source: "ABoxVaryScale2.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.5] },
            GeneratedParam { path: "Fold", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Scale vary", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "R power", kind: ParamKind::Float, offset: 5, default: &[0.33333] },
            GeneratedParam { path: "X,Y,Z power", kind: ParamKind::Float, offset: 6, default: &[6.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).actual_scale = select((*aux).actual_scale, __MB2P0__, ((*aux).i <= 0.0));
		(*aux).actual_scale = ((*aux).actual_scale + ((abs((*aux).actual_scale) - 1.0) * __MB2P4__));
		z.w = (z.w * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__)));
		z.z = ((select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))) * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.z);
		z.y = ((__MB2P3__ * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.y);
		z.x = (((__MB2P3__ + __MB2P3__) * select(select((*aux).actual_scale, ((*aux).actual_scale / exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))))), (1.0 >= exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))))), __MB2P1__, (exp2(((((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634) - round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634))) + round(((log(abs(((exp2(((((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(select(select(z.z, (-(z.z) - (__MB2P3__ + __MB2P3__)), (z.z < select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))), (-(z.z) + (__MB2P3__ + __MB2P3__)), (z.z >= select(select(z.y, (-(z.y) - (__MB2P3__ + __MB2P3__)), (z.y < select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))), (-(z.y) + (__MB2P3__ + __MB2P3__)), (z.y >= select(select(z.x, (-(z.x) - (__MB2P3__ + __MB2P3__)), (z.x < -(__MB2P3__))), (-(z.x) + (__MB2P3__ + __MB2P3__)), (z.x >= -(__MB2P3__))))))))) * __MB2P6__) * 1.4426950408889634)))) + exp2(((((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634) - round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs(__MB2P3__)) * __MB2P6__) * 1.4426950408889634))))) + exp2(((((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634) - round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))) + round(((log(abs((__MB2P3__ + __MB2P3__))) * __MB2P6__) * 1.4426950408889634))))))) * __MB2P5__) * 1.4426950408889634)))) < __MB2P2__))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "ATetraVS2",
        source: "ATetraVS2.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.414] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.5] },
            GeneratedParam { path: "Fold", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Scale vary", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 5, default: &[8.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).actual_scale = select((*aux).actual_scale, __MB2P0__, ((*aux).i <= 0.0));
		(*aux).actual_scale = ((*aux).actual_scale + ((abs((*aux).actual_scale) - 1.0) * __MB2P4__));
		z.y = select(select(z.y, (-((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.x + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.y + (__MB2P3__ * -0.7071067811865476)))));
		z.x = select(select(z.x, (-((((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.y + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.y + (__MB2P3__ * -0.7071067811865476)))));
		z.z = select(select(z.z, (-((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.x + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.z + (__MB2P3__ * -0.7071067811865476)))));
		z.x = select(select(select(select(z.x, (-((((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.y + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.y + (__MB2P3__ * -0.7071067811865476))))), (-((((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.z + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.z + (__MB2P3__ * -0.7071067811865476)))));
		z.z = select(select(select(select(z.z, (-((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.x + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.z + (__MB2P3__ * -0.7071067811865476))))), (-((((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.y + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.y + (__MB2P3__ * -0.7071067811865476)) + (z.z + (__MB2P3__ * -0.7071067811865476)))));
		z.y = select(select(select(select(z.y, (-((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.x + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.x + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.x + (__MB2P3__ * -0.7071067811865476)) + (z.y + (__MB2P3__ * -0.7071067811865476))))), (-((((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))) + (__MB2P3__ * -0.7071067811865476)), (0.0 >= ((((z.y + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) + (((z.z + (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476)) - (__MB2P3__ * -0.7071067811865476))))), (-((z.z + (__MB2P3__ * -0.7071067811865476))) - (__MB2P3__ * -0.7071067811865476)), (0.0 <= ((z.y + (__MB2P3__ * -0.7071067811865476)) + (z.z + (__MB2P3__ * -0.7071067811865476)))));
		z.w = (z.w * select(select((*aux).actual_scale, ((*aux).actual_scale / ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__)), (1.0 >= ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__))), __MB2P1__, (((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__) < __MB2P2__)));
		z.z = ((z.x * select(select((*aux).actual_scale, ((*aux).actual_scale / ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__)), (1.0 >= ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__))), __MB2P1__, (((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__) < __MB2P2__))) + (*aux).const_c.z);
		z.y = ((z.y * select(select((*aux).actual_scale, ((*aux).actual_scale / ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__)), (1.0 >= ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__))), __MB2P1__, (((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__) < __MB2P2__))) + (*aux).const_c.y);
		z.x = ((z.z * select(select((*aux).actual_scale, ((*aux).actual_scale / ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__)), (1.0 >= ((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__))), __MB2P1__, (((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) * __MB2P5__) < __MB2P2__))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Aexion1",
        source: "Aexion1.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "C add", kind: ParamKind::Float, offset: 0, default: &[-1.35] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = select(z.x, (abs(((z.x + z.y) + z.z)) + __MB2P0__), ((*aux).i == 0.0));
		z.y = select(z.y, (abs(((z.z - z.x) - z.y)) + __MB2P0__), ((*aux).i == 0.0));
		z.z = select(z.z, (abs(((z.y - z.x) - z.z)) + __MB2P0__), ((*aux).i == 0.0));
		z.w = select(z.w, (abs(((z.x - z.y) - z.z)) + __MB2P0__), ((*aux).i == 0.0));
		(*aux).const_c.x = select((*aux).const_c.x, (abs((((*aux).const_c.x + (*aux).const_c.y) + (*aux).const_c.z)) + __MB2P0__), ((*aux).i == 0.0));
		(*aux).const_c.y = select((*aux).const_c.y, (abs((((*aux).const_c.z - (*aux).const_c.x) - (*aux).const_c.y)) + __MB2P0__), ((*aux).i == 0.0));
		(*aux).const_c.z = select((*aux).const_c.z, (abs((((*aux).const_c.y - (*aux).const_c.x) - (*aux).const_c.z)) + __MB2P0__), ((*aux).i == 0.0));
		(*aux).const_c.w = select((*aux).const_c.w, (__MB2P0__ + abs((((*aux).const_c.x - (*aux).const_c.y) - (*aux).const_c.z))), ((*aux).i == 0.0));
		z.x = ((((z.x * z.x) - (z.y * z.y)) + ((z.z * z.w) + (z.z * z.w))) + (*aux).const_c.x);
		z.y = ((((z.z * z.w) + (z.z * z.w)) - ((z.x * z.x) - (z.y * z.y))) + (*aux).const_c.y);
		z.z = (((z.w - (((z.x * z.y) + (z.x * z.y)) * ((z.x * z.y) + (z.x * z.y)))) + z.z) + (*aux).const_c.z);
		z.w = ((z.z - (z.w - (((z.x * z.y) + (z.x * z.y)) * ((z.x * z.y) + (z.x * z.y))))) + (*aux).const_c.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "BPine_only",
        source: "BPine_only.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "pixel_x", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "pixel_y", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "pixel_z", kind: ParamKind::Float, offset: 2, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (z.x * z.x);
		t16 = (z.y * z.y);
		t24 = (z.z * z.z);
		t32 = ((z.x + z.x) / sqrt((t16 + t24)));
		z.y = ((((t32 + t32) * z.z) * z.y) + ((*aux).const_c.y * __MB2P2__));
		z.z = ((t32 * (t16 - t24)) + ((*aux).const_c.z * __MB2P1__));
		z.x = (((t8 - t16) - t24) + ((*aux).const_c.x * __MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "BT2Pine",
        source: "BT2Pine.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "pixel_x", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "pixel_y", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "pixel_z", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((t8 + z.y) * (t8 + z.y));
		t8 = ((t8 - z.y) * (t8 - z.y));
		z.z = (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs((sqrt((t16 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs((sqrt((t8 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs((sqrt((t8 + t16)) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		t8 = (z.x * z.x);
		t16 = (z.y * z.y);
		t24 = (z.z * z.z);
		t32 = ((z.x + z.x) / sqrt((t16 + t24)));
		z.y = ((((t32 + t32) * z.z) * z.y) + ((*aux).const_c.y * __MB2P4__));
		z.z = ((t32 * (t16 - t24)) + ((*aux).const_c.z * __MB2P3__));
		z.x = (((t8 - t16) - t24) + ((*aux).const_c.x * __MB2P2__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Barnsley2_4Dc",
        source: "Barnsley2_4Dc.m3f",
        param_floats: 8,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "dx", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "dy", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "dz", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "dw", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "xCy mul", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "yCx mul", kind: ParamKind::Float, offset: 6, default: &[1.0] },
            GeneratedParam { path: "Thresold", kind: ParamKind::Float, offset: 7, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = select((z.x - __MB2P0__), (z.x + __MB2P0__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.y = select((z.y - __MB2P1__), (z.y + __MB2P1__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.z = select((z.z - __MB2P2__), (z.z + __MB2P2__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.w = select(((z.w - __MB2P3__) + __MB2P4__), ((z.w + __MB2P3__) + __MB2P4__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.w = ((((z.w * (*aux).const_c.x) + (z.z * (*aux).const_c.y)) + (z.y * (*aux).const_c.z)) + (z.x * (*aux).const_c.w));
		z.z = ((((z.z * (*aux).const_c.x) + -((z.w * (*aux).const_c.y))) + (z.x * (*aux).const_c.z)) + -((z.y * (*aux).const_c.w)));
		z.y = ((((z.y * (*aux).const_c.x) + (z.x * (*aux).const_c.y)) + -((z.w * (*aux).const_c.z))) + -((z.z * (*aux).const_c.w)));
		z.x = ((((z.x * (*aux).const_c.x) + -((z.y * (*aux).const_c.y))) + -((z.z * (*aux).const_c.z))) + (z.w * (*aux).const_c.w));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Barnsley2_4Dnc",
        source: "Barnsley2_4Dnc.m3f",
        param_floats: 8,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "dx", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "dy", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "dz", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "dw", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "xCy mul", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "yCx mul", kind: ParamKind::Float, offset: 6, default: &[1.0] },
            GeneratedParam { path: "Thresold", kind: ParamKind::Float, offset: 7, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = select((z.x - __MB2P0__), (z.x + __MB2P0__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.y = select((z.y - __MB2P1__), (z.y + __MB2P1__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.z = select((z.z - __MB2P2__), (z.z + __MB2P2__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.w = select(((z.w - __MB2P3__) + __MB2P4__), ((z.w + __MB2P3__) + __MB2P4__), (__MB2P7__ <= (((z.x * (*aux).const_c.y) * __MB2P5__) + ((z.y * (*aux).const_c.x) * __MB2P6__))));
		z.w = ((((z.x * (*aux).const_c.w) + (z.y * (*aux).const_c.z)) + -((z.z * (*aux).const_c.y))) + (z.w * (*aux).const_c.x));
		z.z = ((((z.x * (*aux).const_c.z) + -((z.y * (*aux).const_c.w))) + (z.z * (*aux).const_c.x)) + (z.w * (*aux).const_c.y));
		z.y = ((((z.x * (*aux).const_c.y) + (z.y * (*aux).const_c.x)) + (z.z * (*aux).const_c.w)) + -((z.w * (*aux).const_c.z)));
		z.x = ((((z.x * (*aux).const_c.x) + -((z.y * (*aux).const_c.y))) + -((z.z * (*aux).const_c.z))) + -((z.w * (*aux).const_c.w)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Benesi1Pow2",
        source: "Benesi1Pow2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (abs((*aux).const_c.y) - abs(((z.y * z.y) * (((sqrt(((z.z * z.z) + (z.y * z.y))) * z.x) + (sqrt(((z.z * z.z) + (z.y * z.y))) * z.x)) / ((z.z * z.z) + (z.y * z.y))))));
		z.x = ((((((z.z * z.z) + (z.y * z.y)) - (z.z * z.z)) * (((z.z * z.z) + (z.y * z.y)) - (z.z * z.z))) - z.x) + (*aux).const_c.x);
		z.z = (abs((*aux).const_c.z) - abs(((z.z * (z.y * (((sqrt(((z.z * z.z) + (z.y * z.y))) * z.x) + (sqrt(((z.z * z.z) + (z.y * z.y))) * z.x)) / ((z.z * z.z) + (z.y * z.y))))) + (z.z * (z.y * (((sqrt(((z.z * z.z) + (z.y * z.y))) * z.x) + (sqrt(((z.z * z.z) + (z.y * z.y))) * z.x)) / ((z.z * z.z) + (z.y * z.y))))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Benesi2Pow2",
        source: "Benesi2Pow2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((((z.x * z.x) - (z.z * z.z)) - (z.y * z.y)) + (*aux).const_c.x);
		z.y = (abs((*aux).const_c.y) - abs(((z.x * ((z.y * z.y) + (z.x * z.x))) + (z.x * ((z.y * z.y) + (z.x * z.x))))));
		z.z = (abs((*aux).const_c.z) - abs(((((z.z * z.z) - z.y) * ((z.z * z.z) + (z.z * z.z))) / (z.z * sqrt((z.z * z.z))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Benesi2Pow6",
        source: "Benesi2Pow6.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) - ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))))) + (*aux).const_c.x);
		z.z = (abs((*aux).const_c.z) - (z.x * abs((((z.y * (((((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) + ((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))))) * z.y) + ((((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) + ((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))))) * z.y))) + ((((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) + ((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))))) * ((z.y * z.y) - ((((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) + ((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))))) * (((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) + ((((((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x))))))) - (z.y * z.y)) * (((((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)) * (((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))))) - ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (1.0 / ((((z.y * z.y) + (z.x * z.x)) * ((z.y * z.y) + (z.x * z.x))) * ((z.y * z.y) + (z.x * z.x)))))))))))) * (((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((((z.x * z.x) - ((z.z * z.z) + (z.y * z.y))) * ((z.x * z.x) - ((z.z * z.z) + (z.y * z.y)))) - (((z.x * z.x) * ((z.z * z.z) + (z.y * z.y))) * 12.0))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z))) + ((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x))))) * (((z.y * z.y) + (z.x * z.x)) - (z.z * z.z)))) * (z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x)))))))))));
		z.y = (abs((*aux).const_c.y) - abs((z.z * (sqrt(((z.y * z.y) + (z.x * z.x))) + sqrt(((z.y * z.y) + (z.x * z.x)))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "BenesiPine1",
        source: "BenesiPine1.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = abs((t8 + (z.y * 0.7071067811865475)));
		z.x = abs((t8 + -((z.y * 0.7071067811865475))));
		z.z = abs(((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		t8 = ((z.x + z.y) * 0.7071067811865475);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.y = ((-(z.x) + z.y) * 0.7071067811865475);
		z.x = ((z.x * __MB2P0__) - __MB2P1__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
		t8 = (z.x * z.x);
		t16 = (z.y * z.y);
		t24 = (z.z * z.z);
		t32 = ((z.x + z.x) / sqrt((t16 + t24)));
		z.y = ((((t32 + t32) * z.z) * z.y) + (*aux).const_c.y);
		z.z = ((t32 * (t16 - t24)) + (*aux).const_c.z);
		z.x = (((t8 - t16) - t24) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "BenesiPine2",
        source: "BenesiPine2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((t8 + z.y) * (t8 + z.y));
		t8 = ((t8 - z.y) * (t8 - z.y));
		z.z = (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs((sqrt((t16 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs((sqrt((t8 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs((sqrt((t8 + t16)) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		t8 = (z.x * z.x);
		t16 = (z.y * z.y);
		t24 = (z.z * z.z);
		t32 = ((z.x + z.x) / sqrt((t16 + t24)));
		z.y = ((((t32 + t32) * z.z) * z.y) + (*aux).const_c.y);
		z.z = ((t32 * (t16 - t24)) + (*aux).const_c.z);
		z.x = (((t8 - t16) - t24) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Beth322",
        source: "Beth322.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((((((z.x * z.x) * (z.x * z.x)) + ((z.x * z.x) * (z.x * z.x))) - ((((((z.z * z.z) + (z.y * z.y)) * (z.x * z.x)) + (((z.z * z.z) + (z.y * z.y)) * (z.x * z.x))) - (((z.z * z.z) * (z.z * z.z)) + ((z.y * z.y) * (z.y * z.y)))) * 5.0)) * z.x) + (*aux).const_c.x);
		z.y = ((((((z.y * z.y) * (z.y * z.y)) + ((z.y * z.y) * (z.y * z.y))) - ((((((z.z * z.z) + (z.x * z.x)) * (z.y * z.y)) + (((z.z * z.z) + (z.x * z.x)) * (z.y * z.y))) - (((z.z * z.z) * (z.z * z.z)) + ((z.x * z.x) * (z.x * z.x)))) * 5.0)) * z.y) + (*aux).const_c.y);
		z.z = ((((((z.z * z.z) * (z.z * z.z)) + ((z.z * z.z) * (z.z * z.z))) - ((((z.z * z.z) * ((((z.x * z.x) * (z.x * z.x)) + (z.x * z.x)) + (((z.x * z.x) * (z.x * z.x)) + (z.x * z.x)))) - ((z.y * z.y) + ((z.y * z.y) * (z.y * z.y)))) * 5.0)) * z.z) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Beth323",
        source: "Beth323.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		z.x = (z.x * z.x);
		z.y = (z.y * z.y);
		z.z = (z.z * z.z);
		t32 = (z.y * z.y);
		t40 = (z.z * z.z);
		t8 = ((((((((z.y - z.z) * z.x) * z.x) * 21.0) + ((((z.y * z.y) - (z.z * z.z)) * z.x) * -35.0)) + (((t32 * z.y) - (t40 * z.z)) * 7.0)) * t8) + (*aux).const_c.x);
		t32 = (z.z * z.z);
		t40 = (z.x * z.x);
		t16 = ((((((((z.z - z.x) * z.y) * z.y) * 21.0) + ((((z.z * z.z) - (z.x * z.x)) * z.y) * -35.0)) + (((t32 * z.z) - (t40 * z.x)) * 7.0)) * t16) + (*aux).const_c.y);
		t32 = (z.x * z.x);
		t40 = (z.y * z.y);
		t24 = ((((((((z.x - z.y) * z.z) * z.z) * 21.0) + ((((z.x * z.x) - (z.y * z.y)) * z.z) * -35.0)) + (((t32 * z.x) - (t40 * z.y)) * 7.0)) * t24) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Beth432",
        source: "Beth432.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((((((((((((((z.y * z.y) * (z.y * z.y)) * (z.z * z.z)) + ((z.y * z.y) * ((z.z * z.z) * (z.z * z.z)))) * -105.0) + (((((((z.y * z.y) * (z.z * z.z)) * -12.0) + ((z.y * z.y) * (z.y * z.y))) + ((z.z * z.z) * (z.z * z.z))) * (z.x * z.x)) * -35.0)) + ((((z.z * z.z) + (z.y * z.y)) * ((z.x * z.x) * (z.x * z.x))) * -21.0)) + (((z.x * z.x) * (z.x * z.x)) * (z.x * z.x))) + (((z.x * z.x) * (z.x * z.x)) * (z.x * z.x))) + ((((z.y * z.y) * (z.y * z.y)) * (z.y * z.y)) * 14.0)) + ((((z.z * z.z) * (z.z * z.z)) * (z.z * z.z)) * 14.0)) * -0.3950617283950617) * z.x) + (*aux).const_c.x);
		z.y = (((((((((((((z.z * z.z) * (z.z * z.z)) * (z.x * z.x)) + ((z.z * z.z) * ((z.x * z.x) * (z.x * z.x)))) * -105.0) + (((((((z.x * z.x) * (z.z * z.z)) * -12.0) + ((z.x * z.x) * (z.x * z.x))) + ((z.z * z.z) * (z.z * z.z))) * (z.y * z.y)) * -35.0)) + ((((z.z * z.z) + (z.x * z.x)) * ((z.y * z.y) * (z.y * z.y))) * -21.0)) + ((((z.y * z.y) * (z.y * z.y)) * (z.y * z.y)) + (((z.y * z.y) * (z.y * z.y)) * (z.y * z.y)))) + ((((z.x * z.x) * (z.x * z.x)) * (z.x * z.x)) * 14.0)) + ((((z.z * z.z) * (z.z * z.z)) * (z.z * z.z)) * 14.0)) * -0.3950617283950617) * z.y) + (*aux).const_c.y);
		z.z = ((((((z.z * z.z) + ((((z.y * z.y) * ((z.y * z.y) * (z.y * z.y))) + ((z.x * z.x) * (((((((z.x * z.x) * (z.x * z.x)) * (z.y * z.y)) + ((z.x * z.x) * ((z.y * z.y) * (z.y * z.y)))) * -105.0) + (((((((z.x * z.x) * (z.y * z.y)) * -12.0) + ((z.x * z.x) * (z.x * z.x))) + ((z.y * z.y) * (z.y * z.y))) * (z.z * z.z)) * -35.0)) + ((((z.y * z.y) + (z.x * z.x)) * ((z.z * z.z) * (z.z * z.z))) * -21.0)))) * 14.0)) + ((((z.z * z.z) * (z.z * z.z)) * ((z.x * z.x) * (z.x * z.x))) + (((z.z * z.z) * (z.z * z.z)) * ((z.x * z.x) * (z.x * z.x))))) * -0.3950617283950617) * z.z) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Beth641",
        source: "Beth641.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (((((((((z.w * z.w) + (z.z * z.z)) + (z.y * z.y)) * (z.x * z.x)) * -10.0) + (((((z.w * z.w) * (z.w * z.w)) + ((z.y * z.y) * (z.y * z.y))) + ((z.z * z.z) * (z.z * z.z))) * 5.0)) + (((z.x * z.x) * (z.x * z.x)) * 3.0)) * z.x) + (*aux).const_c.x);
		z.y = (((((((((z.w * z.w) * (z.w * z.w)) + ((z.x * z.x) * (z.x * z.x))) + ((z.z * z.z) * (z.z * z.z))) * 5.0) + (((((z.w * z.w) + (z.z * z.z)) + (z.x * z.x)) * (z.y * z.y)) * -10.0)) + (((z.y * z.y) * (z.y * z.y)) * 3.0)) * z.y) + (*aux).const_c.y);
		z.z = (((((((((z.w * z.w) * (z.w * z.w)) + ((z.x * z.x) * (z.x * z.x))) + ((z.y * z.y) * (z.y * z.y))) * 5.0) + (((((z.w * z.w) + (z.y * z.y)) + (z.x * z.x)) * (z.z * z.z)) * -10.0)) + (((z.z * z.z) * (z.z * z.z)) * 3.0)) * z.z) + (*aux).const_c.z);
		z.w = (((((z.w * z.w) + (((((((z.z * z.z) * (z.z * z.z)) + (((z.y * z.y) * (z.y * z.y)) + ((z.x * z.x) * (z.x * z.x)))) * 5.0) + (((z.w * z.w) * (z.w * z.w)) * 3.0)) * (((z.z * z.z) + (z.x * z.x)) + (z.y * z.y))) * -10.0)) * z.w) + (*aux).const_c.w) + __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "BristorBrot",
        source: "BristorBrot.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Z mul", kind: ParamKind::Float, offset: 2, default: &[-1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((z.x * __MB2P0__) + (z.y * __MB2P1__)) * z.z) + (*aux).const_c.z);
		z.y = ((((z.x * __MB2P0__) + (z.z * __MB2P2__)) * z.y) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) + -((z.y * z.y))) + -((z.z * z.z))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "BulboxP-2",
        source: "BulboxP-2.m3f",
        param_floats: 10,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Fold", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Inner R", kind: ParamKind::Float, offset: 2, default: &[0.6] },
            GeneratedParam { path: "Inner Scale", kind: ParamKind::Float, offset: 3, default: &[-0.5] },
            GeneratedParam { path: "Unsharpening", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "Inv-xCen", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "Inv-yCen", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "Inv-zCen", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "Inner Z Mul", kind: ParamKind::Float, offset: 8, default: &[1.0] },
            GeneratedParam { path: "Disable box", kind: ParamKind::Float, offset: 9, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		z.x = (abs((z.x + __MB2P1__)) + (-(abs((z.x - __MB2P1__))) - z.x));
		z.y = (abs((z.y + __MB2P1__)) + (-(abs((z.y - __MB2P1__))) - z.y));
		z.z = (abs((z.z + __MB2P1__)) + (-(abs((z.z - __MB2P1__))) - z.z));
		t24 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t32 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		z.x = ((z.x * __MB2P0__) - __MB2P5__);
		z.y = ((z.y * __MB2P0__) - __MB2P6__);
		z.z = ((z.z * __MB2P0__) - __MB2P7__);
		t40 = select(t40, ((t32 - __MB2P2__) / (1.0 - __MB2P2__)), (__MB2P2__ < t32));
		t48 = select(t48, (1.0 - ((t32 - __MB2P2__) / (1.0 - __MB2P2__))), (__MB2P2__ < t32));
		t8 = sqrt((((z.y * z.y) + (z.x * z.x)) + abs(__MB2P4__)));
		t24 = (__MB2P3__ / (t24 * t24));
		t16 = (-(((z.z / t8) * (z.z / t8))) + 1.0);
		z.z = select(((((-((z.z + z.z)) * t8) * t24) * __MB2P8__) + (*aux).const_c.z), ((((((-((z.z + z.z)) * __MB2P8__) * t8) * t24) * t48) + (z.z * t40)) + (*aux).const_c.z), (__MB2P2__ < t32));
		z.y = select((((-((z.x * z.y)) * t16) * t24) + (*aux).const_c.y), (((((-((z.x * z.y)) * t16) * t24) * t48) + (z.y * t40)) + (*aux).const_c.y), (__MB2P2__ < t32));
		z.x = select((((((z.x * z.x) + -((z.y * z.y))) * t16) * t24) + (*aux).const_c.x), (((((((z.x * z.x) + -((z.y * z.y))) * t16) * t24) * t48) + (z.x * t40)) + (*aux).const_c.x), (__MB2P2__ < t32));
		z.x = (z.x + __MB2P5__);
		z.y = (z.y + __MB2P6__);
		z.z = (z.z + __MB2P7__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "CommQuat",
        source: "CommQuat.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Z mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "W mul", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (((((z.x * z.x) - (z.y * z.y)) - ((z.w * z.z) + (z.w * z.z))) * __MB2P0__) + (*aux).const_c.x);
		z.y = ((((((z.x * z.y) + (z.x * z.y)) + (z.z * z.z)) - (z.w * z.w)) * __MB2P1__) + (*aux).const_c.y);
		z.z = (((((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w))) * __MB2P2__) + (*aux).const_c.z);
		z.w = ((((((z.w * z.x) + (z.z * z.y)) + ((z.w * z.x) + (z.z * z.y))) * __MB2P3__) + __MB2P4__) + (*aux).const_c.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "CosinePow2",
        source: "CosinePow2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Z multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((z.z * z.z) - ((z.x * z.x) + (z.y * z.y))) * __MB2P0__) + (*aux).const_c.z);
		z.y = ((((((z.z + z.z) / sqrt(((z.x * z.x) + (z.y * z.y)))) * z.x) * z.y) + ((((z.z + z.z) / sqrt(((z.x * z.x) + (z.y * z.y)))) * z.x) * z.y)) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) - (z.y * z.y)) * ((z.z + z.z) / sqrt(((z.x * z.x) + (z.y * z.y))))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "CosinePow8",
        source: "CosinePow8.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Z multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((z.z * z.z) * ((((((((z.z * z.z) * (z.z * z.z)) * ((z.z * z.z) * (z.z * z.z))) - (((((((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y))) + ((z.z * z.z) * (z.z * z.z))) * 28.0) * ((z.x * z.x) + (z.y * z.y))) * (z.z * z.z))) + ((((z.z * z.z) * (z.z * z.z)) * (((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y)))) * 70.0)) + ((((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y))) * (((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y))))) * __MB2P0__) + (*aux).const_c.z));
		z.y = (((((((((z.x * z.x) * 7.0) - (z.y * z.y)) * ((z.y * z.y) * (z.y * z.y))) * 8.0) * z.x) * z.y) * (((z.x * z.x) + (z.y * z.y)) / ((((((((((((z.x * z.x) + (z.y * z.y)) - (z.z * z.z)) * 7.0) * (z.z * z.z)) - (((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y)))) * ((z.x * z.x) + (z.y * z.y))) + ((z.z * z.z) * (z.z * z.z))) * z.z) * 8.0) * ((((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y))) * sqrt(((z.x * z.x) + (z.y * z.y))))) + 1e-40))) + (*aux).const_c.y);
		z.x = (((((z.x * z.x) + ((((((((z.x * z.x) - ((z.y * z.y) * 7.0)) * ((z.x * z.x) * (z.x * z.x))) + ((z.x * z.x) * (z.x * z.x))) * 70.0) + ((z.y * z.y) * (z.y * z.y))) * ((z.y * z.y) * (z.y * z.y))) * (((((((z.x * z.x) - ((z.y * z.y) * 7.0)) * ((z.x * z.x) * (z.x * z.x))) + ((z.x * z.x) * (z.x * z.x))) * 70.0) + ((z.y * z.y) * (z.y * z.y))) * ((z.y * z.y) * (z.y * z.y))))) - ((((z.y * z.y) * (((z.y * z.y) * (z.y * z.y)) + ((((z.x * z.x) - ((z.y * z.y) * 7.0)) * ((z.x * z.x) * (z.x * z.x))) + ((z.x * z.x) * (z.x * z.x))))) * ((((z.x * z.x) - ((z.y * z.y) * 7.0)) * ((z.x * z.x) * (z.x * z.x))) + ((z.x * z.x) * (z.x * z.x)))) * 28.0)) * (((z.x * z.x) + (z.y * z.y)) / ((((((((((((z.x * z.x) + (z.y * z.y)) - (z.z * z.z)) * 7.0) * (z.z * z.z)) - (((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y)))) * ((z.x * z.x) + (z.y * z.y))) + ((z.z * z.z) * (z.z * z.z))) * z.z) * 8.0) * ((((z.x * z.x) + (z.y * z.y)) * ((z.x * z.x) + (z.y * z.y))) * sqrt(((z.x * z.x) + (z.y * z.y))))) + 1e-40))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "CubicQuat",
        source: "CubicQuat.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Z mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "W mul", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "X twist", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "Y twist", kind: ParamKind::Float, offset: 6, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((((z.x * z.w) * __MB2P3__) + ((z.x * z.w) * __MB2P3__)) + __MB2P4__) + (*aux).const_c.w);
		z.z = ((((z.x * z.z) * __MB2P2__) + ((z.x * z.z) * __MB2P2__)) + (*aux).const_c.z);
		z.y = (((((z.x * z.y) * __MB2P1__) + ((z.x * z.y) * __MB2P1__)) + (*aux).const_c.y) + ((*aux).const_c.x * __MB2P6__));
		z.x = (((-((((z.y * z.y) * __MB2P1__) + (((z.z * z.z) * __MB2P2__) + ((z.w * z.w) * __MB2P3__)))) + ((z.x * z.x) * __MB2P0__)) + (*aux).const_c.x) + ((*aux).const_c.y * __MB2P5__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Faehrten",
        source: "Faehrten.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Y multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Z multiplier", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Squares multiplier", kind: ParamKind::Float, offset: 2, default: &[-0.3333333333333333] },
            GeneratedParam { path: "Twist", kind: ParamKind::Float, offset: 3, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((z.z * z.z) + (((z.x * z.x) + (z.y * z.y)) * __MB2P2__)) + ((z.x * z.y) * __MB2P3__)) + (*aux).const_c.z);
		z.y = ((((z.y * z.y) + (((z.x * z.x) + (z.z * z.z)) * __MB2P2__)) + ((z.x * z.z) * __MB2P3__)) + (*aux).const_c.y);
		z.x = ((z.x * ((z.y * __MB2P0__) + (z.z * __MB2P1__))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Hopf4D",
        source: "Hopf4D.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "z mul", kind: ParamKind::Float, offset: 1, default: &[-1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (-((((z.z * z.z) + (z.w * z.w)) / ((z.x * z.x) + (z.y * z.y)))) + 1.0);
		t16 = (sqrt((((z.x * z.x) + (z.y * z.y)) / ((z.z * z.z) + (z.w * z.w)))) + sqrt((((z.x * z.x) + (z.y * z.y)) / ((z.z * z.z) + (z.w * z.w)))));
		z.w = ((((z.z * z.w) * t16) + (*aux).const_c.w) + __MB2P0__);
		z.z = (((((z.z * z.z) - (z.w * z.w)) * t16) * __MB2P1__) + (*aux).const_c.z);
		z.y = ((((z.x * z.y) * t8) + ((z.x * z.y) * t8)) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) - (z.y * z.y)) * t8) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IQ-bulb",
        source: "IQ-bulb.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-AnglePow", kind: ParamKind::Float, offset: 1, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Mode", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Atan2Mode", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t16 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		t24 = sqrt(((z.x * z.x) + (z.z * z.z)));
		t40 = cos((atan2(sqrt(((z.x * z.x) + (z.z * z.z))), z.y) * __MB2P0__));
		t32 = sin((atan2(sqrt(((z.x * z.x) + (z.z * z.z))), z.y) * __MB2P0__));
		t56 = cos((atan2(z.x, z.z) * __MB2P1__));
		t48 = sin((atan2(z.x, z.z) * __MB2P1__));
		z.x = (((t32 * t48) * t16) + (*aux).const_c.x);
		z.y = ((t40 * t16) + (*aux).const_c.y);
		z.z = ((((t32 * t56) * __MB2P2__) * t16) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IQ_NormBulb",
        source: "IQ_NormBulb.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-AnglePow", kind: ParamKind::Float, offset: 1, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Mode", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Atan2Mode", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t16 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		t24 = sqrt(((z.y * z.y) + (z.z * z.z)));
		t40 = cos((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t32 = sin((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t56 = cos((atan2(z.y, z.z) * __MB2P1__));
		t48 = sin((atan2(z.y, z.z) * __MB2P1__));
		z.x = ((t32 * t16) + (*aux).const_c.x);
		z.y = (((t40 * t48) * t16) + (*aux).const_c.y);
		z.z = ((((t40 * t56) * __MB2P2__) * t16) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IQ_NormBulb_NoYZ",
        source: "IQ_NormBulb_NoYZ.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-AnglePow", kind: ParamKind::Float, offset: 1, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Mode", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Atan2Mode", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t16 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		t24 = sqrt(((z.y * z.y) + (z.z * z.z)));
		t40 = cos((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t32 = sin((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t56 = cos((atan2(z.y, z.z) * __MB2P1__));
		t48 = sin((atan2(z.y, z.z) * __MB2P1__));
		z.x = ((t32 * t16) + (*aux).const_c.x);
		z.y = ((t40 * t48) * t16);
		z.z = (((t40 * t56) * __MB2P2__) * t16);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IQ_PineTree",
        source: "IQ_PineTree.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-AnglePow", kind: ParamKind::Float, offset: 1, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Mode", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Atan2Mode", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t16 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		t24 = sqrt(((z.y * z.y) + (z.z * z.z)));
		t40 = cos((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t32 = sin((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t56 = cos((atan2(z.y, z.z) * __MB2P1__));
		t48 = sin((atan2(z.y, z.z) * __MB2P1__));
		z.x = ((t40 * t16) + (*aux).const_c.x);
		z.y = (((t32 * t56) * t16) + (*aux).const_c.y);
		z.z = ((((t32 * t48) * __MB2P2__) * t16) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IQ_PineTree_NoYZ",
        source: "IQ_PineTree_NoYZ.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-AnglePow", kind: ParamKind::Float, offset: 1, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Mode", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Atan2Mode", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t16 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		t24 = sqrt(((z.y * z.y) + (z.z * z.z)));
		t40 = cos((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t32 = sin((atan2(sqrt(((z.y * z.y) + (z.z * z.z))), z.x) * __MB2P0__));
		t56 = cos((atan2(z.y, z.z) * __MB2P1__));
		t48 = sin((atan2(z.y, z.z) * __MB2P1__));
		z.x = ((t40 * t16) + (*aux).const_c.x);
		z.y = ((t32 * t56) * t16);
		z.z = (((t32 * t48) * __MB2P2__) * t16);
	return z;
"####,
    },
    GeneratedFormula {
        name: "IdesFormula",
        source: "IdesFormula.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Z mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Xsub mul", kind: ParamKind::Float, offset: 3, default: &[0.5] },
            GeneratedParam { path: "Zsub mul", kind: ParamKind::Float, offset: 4, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((((z.x * z.y) * z.z) * __MB2P1__) + (*aux).const_c.y);
		z.x = ((((z.y * z.y) * __MB2P0__) - ((((z.x * z.x) * (z.x * z.x)) + z.z) * __MB2P3__)) + (*aux).const_c.x);
		z.z = ((z.z - (((((z.x * z.x) * (z.x * z.x)) * __MB2P2__) + (z.y * z.y)) * __MB2P4__)) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "JCube3",
        source: "JCube3.m3f",
        param_floats: 9,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Alpha 1", kind: ParamKind::Float, offset: 0, default: &[0.41421356] },
            GeneratedParam { path: "Alpha 2", kind: ParamKind::Float, offset: 1, default: &[0.41421356] },
            GeneratedParam { path: "Cx1", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Cy1", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Cz1", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "Cx2", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "Cy2", kind: ParamKind::Float, offset: 6, default: &[1.0] },
            GeneratedParam { path: "Cz2", kind: ParamKind::Float, offset: 7, default: &[1.0] },
            GeneratedParam { path: "GScale (test)", kind: ParamKind::Float, offset: 8, default: &[3.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t0: f32 = 0.0;
		var t8: f32 = 0.0;
		var t16: f32 = 0.0;
		var t24: f32 = 0.0;
		t24 = -(__MB2P8__);
		t0 = ((__MB2P8__ - 1.0) + __MB2P0__);
		t8 = (((__MB2P8__ - 1.0) + __MB2P0__) * __MB2P1__);
		t16 = (1.0 / (((__MB2P8__ - 1.0) + __MB2P0__) * __MB2P1__));
		t24 = (((1.0 / (((__MB2P8__ - 1.0) + __MB2P0__) * __MB2P1__)) * t24) + 1.0);
		z.w = select((t0 * z.w), select(z.w, (t8 * z.w), (select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) > (select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) + t24))), (t16 > select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y))))))));
		z.x = select((((select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) - __MB2P5__) * t0) + __MB2P5__), select(select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))), (((select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) - __MB2P2__) * t8) + __MB2P2__), (select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) > (select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) + t24))), (t16 > select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y))))))));
		z.y = select((((select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) - __MB2P6__) * t0) + __MB2P6__), select(select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))), (((select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) - __MB2P3__) * t8) + __MB2P3__), (select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) > (select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) + t24))), (t16 > select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y))))))));
		z.z = select((((z.z - __MB2P7__) * t0) + __MB2P7__), select(z.z, (((z.z - __MB2P4__) * t8) + __MB2P4__), (select(select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) > (select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))))) + t24))), (t16 > select(select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))), (select(select(z.y, z.x, (z.x >= z.y)), select(z.x, z.y, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y)))) >= select(select(z.x, z.y, (z.x >= z.y)), select(z.y, z.x, (z.x >= z.y)), (select(z.x, z.y, (z.x >= z.y)) < select(z.y, z.x, (z.x >= z.y))))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Kalisets1",
        source: "Kalisets1.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 2.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Fix", kind: ParamKind::Float, offset: 1, default: &[1e-60] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((abs(z.x) * (__MB2P0__ / ((((abs(z.x) * abs(z.x)) + (abs(z.y) * abs(z.y))) + (abs(z.z) * abs(z.z))) + __MB2P1__))) + (*aux).const_c.x);
		z.y = ((abs(z.y) * (__MB2P0__ / ((((abs(z.x) * abs(z.x)) + (abs(z.y) * abs(z.y))) + (abs(z.z) * abs(z.z))) + __MB2P1__))) + (*aux).const_c.y);
		z.z = ((abs(z.z) * (__MB2P0__ / ((((abs(z.x) * abs(z.x)) + (abs(z.y) * abs(z.y))) + (abs(z.z) * abs(z.z))) + __MB2P1__))) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Lambda4Dc",
        source: "Lambda4Dc.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "dx", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "dy", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "dz", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "dw", kind: ParamKind::Float, offset: 3, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((-((z.x * z.w)) + -((z.x * z.w))) + (-((z.y * z.z)) + -((z.y * z.z)))) + __MB2P3__);
		z.z = (((-((z.x * z.z)) + -((z.x * z.z))) + ((z.y * z.w) + (z.y * z.w))) + __MB2P2__);
		z.y = (((-((z.x * z.y)) + -((z.x * z.y))) + ((z.z * z.w) + (z.z * z.w))) + __MB2P1__);
		z.x = ((((-((z.x * z.x)) + (z.y * z.y)) + (z.z * z.z)) + -((z.w * z.w))) + __MB2P0__);
		z.w = ((((z.w * (*aux).const_c.x) + (z.z * (*aux).const_c.y)) + (z.y * (*aux).const_c.z)) + (z.x * (*aux).const_c.w));
		z.z = ((((z.z * (*aux).const_c.x) + -((z.w * (*aux).const_c.y))) + (z.x * (*aux).const_c.z)) + -((z.y * (*aux).const_c.w)));
		z.y = ((((z.y * (*aux).const_c.x) + (z.x * (*aux).const_c.y)) + -((z.w * (*aux).const_c.z))) + -((z.z * (*aux).const_c.w)));
		z.x = ((((z.x * (*aux).const_c.x) + -((z.y * (*aux).const_c.y))) + -((z.z * (*aux).const_c.z))) + (z.w * (*aux).const_c.w));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Lambda4Dnc",
        source: "Lambda4Dnc.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "dx", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "dy", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "dz", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "dw", kind: ParamKind::Float, offset: 3, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = ((-((z.x * z.w)) + -((z.x * z.w))) + __MB2P3__);
		z.z = ((-((z.x * z.z)) + -((z.x * z.z))) + __MB2P2__);
		z.y = ((-((z.x * z.y)) + -((z.x * z.y))) + __MB2P1__);
		z.x = ((((-((z.x * z.x)) + (z.y * z.y)) + (z.z * z.z)) + (z.w * z.w)) + __MB2P0__);
		z.w = ((((z.x * (*aux).const_c.w) + (z.y * (*aux).const_c.z)) + -((z.z * (*aux).const_c.y))) + (z.w * (*aux).const_c.x));
		z.z = ((((z.x * (*aux).const_c.z) + -((z.y * (*aux).const_c.w))) + (z.z * (*aux).const_c.x)) + (z.w * (*aux).const_c.y));
		z.y = ((((z.x * (*aux).const_c.y) + (z.y * (*aux).const_c.x)) + (z.z * (*aux).const_c.w)) + -((z.w * (*aux).const_c.z)));
		z.x = ((((z.x * (*aux).const_c.x) + -((z.y * (*aux).const_c.y))) + -((z.z * (*aux).const_c.z))) + -((z.w * (*aux).const_c.w)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "MagVsXYZ",
        source: "MagVsXYZ.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Axiom 1", kind: ParamKind::Float, offset: 1, default: &[1.5707963] },
            GeneratedParam { path: "Axiom 2", kind: ParamKind::Float, offset: 2, default: &[0.78539816] },
            GeneratedParam { path: "CosShift", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CxM", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CyM", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "CzM", kind: ParamKind::Float, offset: 6, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t72: f32 = 0.0;
		var t64: f32 = 0.0;
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = (__MB2P0__ * __MB2P1__);
		t72 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t48 = sqrt((((z.x * z.x) * t8) + t72));
		t56 = sqrt((((z.y * z.y) * t8) + t72));
		t64 = sqrt((((z.z * z.z) * t8) + t72));
		t48 = exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48)))));
		t56 = exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56)))));
		t64 = exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64)))));
		t24 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.x) * t16));
		t32 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.y) * t16));
		t40 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.z) * t16));
		z.x = ((cos(((t24 * __MB2P0__) + __MB2P3__)) * t48) + ((*aux).const_c.x * __MB2P3__));
		z.y = ((cos(((t32 * __MB2P0__) + __MB2P3__)) * t56) + ((*aux).const_c.y * __MB2P4__));
		z.z = ((cos(((t40 * __MB2P0__) + __MB2P3__)) * t64) + ((*aux).const_c.z * __MB2P5__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "MagVsXYZabs",
        source: "MagVsXYZabs.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Axiom 1", kind: ParamKind::Float, offset: 1, default: &[1.5707963] },
            GeneratedParam { path: "Axiom 2", kind: ParamKind::Float, offset: 2, default: &[1.5707963] },
            GeneratedParam { path: "CosShift", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CxM", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CyM", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "CzM", kind: ParamKind::Float, offset: 6, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t72: f32 = 0.0;
		var t64: f32 = 0.0;
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = (__MB2P0__ * __MB2P1__);
		t72 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t48 = sqrt((((z.x * z.x) * t8) + t72));
		t56 = sqrt((((z.y * z.y) * t8) + t72));
		t64 = sqrt((((z.z * z.z) * t8) + t72));
		t48 = exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48)))));
		t56 = exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56)))));
		t64 = exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64)))));
		t24 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.x) * t16));
		t32 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.y) * t16));
		t40 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.z) * t16));
		z.x = abs(((cos(((t24 * __MB2P1__) + __MB2P3__)) * t48) + ((*aux).const_c.x * __MB2P3__)));
		z.y = abs(((cos(((t32 * __MB2P1__) + __MB2P3__)) * t56) + ((*aux).const_c.y * __MB2P4__)));
		z.z = abs(((cos(((t40 * __MB2P1__) + __MB2P3__)) * t64) + ((*aux).const_c.z * __MB2P5__)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "MagVsXYZabs2",
        source: "MagVsXYZabs2.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Axiom 1", kind: ParamKind::Float, offset: 1, default: &[1.5707963] },
            GeneratedParam { path: "Axiom 2", kind: ParamKind::Float, offset: 2, default: &[1.5707963] },
            GeneratedParam { path: "CosShift", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CxM", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CyM", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "CzM", kind: ParamKind::Float, offset: 6, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t72: f32 = 0.0;
		var t64: f32 = 0.0;
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = (__MB2P0__ * __MB2P1__);
		t72 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t48 = sqrt((((z.x * z.x) * t8) + t72));
		t56 = sqrt((((z.y * z.y) * t8) + t72));
		t64 = sqrt((((z.z * z.z) * t8) + t72));
		t48 = exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48)))));
		t56 = exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56)))));
		t64 = exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64)))));
		t24 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.x) * t16));
		t32 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.y) * t16));
		t40 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.z) * t16));
		z.x = select(((cos(((t24 * __MB2P1__) + __MB2P3__)) * t48) - ((*aux).const_c.x * __MB2P3__)), ((cos(((t24 * __MB2P1__) + __MB2P3__)) * t48) + ((*aux).const_c.x * __MB2P3__)), (0.0 == ((*aux).const_c.x * __MB2P3__)));
		z.y = select(((cos(((t32 * __MB2P1__) + __MB2P3__)) * t56) - ((*aux).const_c.y * __MB2P4__)), ((cos(((t32 * __MB2P1__) + __MB2P3__)) * t56) + ((*aux).const_c.y * __MB2P4__)), (0.0 == ((*aux).const_c.y * __MB2P4__)));
		z.z = select(((cos(((t40 * __MB2P1__) + __MB2P3__)) * t64) - ((*aux).const_c.z * __MB2P5__)), ((cos(((t40 * __MB2P1__) + __MB2P3__)) * t64) + ((*aux).const_c.z * __MB2P5__)), (0.0 == ((*aux).const_c.z * __MB2P5__)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "MagVsXYZabs3",
        source: "MagVsXYZabs3.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Axiom 1", kind: ParamKind::Float, offset: 1, default: &[1.5707963] },
            GeneratedParam { path: "Axiom 2", kind: ParamKind::Float, offset: 2, default: &[1.5707963] },
            GeneratedParam { path: "CosShift", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CxM", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CyM", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "CzM", kind: ParamKind::Float, offset: 6, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t72: f32 = 0.0;
		var t64: f32 = 0.0;
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = (__MB2P0__ * __MB2P1__);
		t72 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t48 = sqrt((((z.x * z.x) * t8) + t72));
		t56 = sqrt((((z.y * z.y) * t8) + t72));
		t64 = sqrt((((z.z * z.z) * t8) + t72));
		t48 = exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48)))));
		t56 = exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56)))));
		t64 = exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64)))));
		t24 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.x) * t16));
		t32 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.y) * t16));
		t40 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.z) * t16));
		z.x = (select((cos(((t24 * __MB2P0__) + __MB2P3__)) * t48), -((cos(((t24 * __MB2P0__) + __MB2P3__)) * t48)), (0.0 != z.x)) + ((*aux).const_c.x * __MB2P3__));
		z.y = (select((cos(((t32 * __MB2P0__) + __MB2P3__)) * t56), -((cos(((t32 * __MB2P0__) + __MB2P3__)) * t56)), (0.0 != z.y)) + ((*aux).const_c.y * __MB2P4__));
		z.z = (select((cos(((t40 * __MB2P0__) + __MB2P3__)) * t64), -((cos(((t40 * __MB2P0__) + __MB2P3__)) * t64)), (0.0 != z.z)) + ((*aux).const_c.z * __MB2P5__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "MagVsXYZmsw",
        source: "MagVsXYZmsw.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Axiom 1", kind: ParamKind::Float, offset: 1, default: &[1.5707963] },
            GeneratedParam { path: "Axiom 2", kind: ParamKind::Float, offset: 2, default: &[1.5707963] },
            GeneratedParam { path: "CosShift", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CxM", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CyM", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "CzM", kind: ParamKind::Float, offset: 6, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t72: f32 = 0.0;
		var t64: f32 = 0.0;
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = (__MB2P0__ * __MB2P1__);
		t72 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t48 = sqrt((((z.x * z.x) * t8) + t72));
		t56 = sqrt((((z.y * z.y) * t8) + t72));
		t64 = sqrt((((z.z * z.z) * t8) + t72));
		t48 = exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48)))));
		t56 = exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56)))));
		t64 = exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64)))));
		t24 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.x) * t16));
		t32 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.y) * t16));
		t40 = atan2(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))), (abs(z.z) * t16));
		t48 = select(exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48))))), select(exp2((((__MB2P0__ * log2(abs(t48))) % 1.0) + (__MB2P0__ * log2(abs(t48))))), -(t48), (0.0 > z.x)), (0.0 > (*aux).const_c.x));
		t56 = select(exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56))))), select(exp2((((__MB2P0__ * log2(abs(t56))) % 1.0) + (__MB2P0__ * log2(abs(t56))))), -(t56), (0.0 > z.y)), (0.0 > (*aux).const_c.y));
		t64 = select(exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64))))), select(exp2((((__MB2P0__ * log2(abs(t64))) % 1.0) + (__MB2P0__ * log2(abs(t64))))), -(t64), (0.0 > z.z)), (0.0 > (*aux).const_c.z));
		z.x = ((cos(((t24 * __MB2P0__) + __MB2P3__)) * t48) + ((*aux).const_c.x * __MB2P3__));
		z.y = ((cos(((t32 * __MB2P0__) + __MB2P3__)) * t56) + ((*aux).const_c.y * __MB2P4__));
		z.z = ((cos(((t40 * __MB2P0__) + __MB2P3__)) * t64) + ((*aux).const_c.z * __MB2P5__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin-NSR-tri",
        source: "Makin-NSR-tri.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[3.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 1, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = cos((atan2(z.y, z.x) * __MB2P0__));
		t8 = sin((atan2(z.y, z.x) * __MB2P0__));
		t32 = cos((atan2(z.z, z.y) * __MB2P0__));
		t24 = sin((atan2(z.z, z.y) * __MB2P0__));
		t48 = cos((atan2(z.x, z.z) * __MB2P0__));
		t40 = sin((atan2(z.x, z.z) * __MB2P0__));
		t56 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		z.z = (t24 * t48);
		z.x = (t16 * t40);
		z.y = (t8 * t32);
		z.x = ((z.x * (t56 / sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))) + (*aux).const_c.x);
		z.y = ((z.y * (t56 / sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))) + (*aux).const_c.y);
		z.z = ((((t56 / sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))) * z.z) * __MB2P1__) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin3D-1",
        source: "Makin3D-1.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((z.z + z.z) * (z.x - z.y)) + (*aux).const_c.z);
		z.y = (((z.x * z.y) + (z.x * z.y)) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) - (z.y * z.y)) - (z.z * z.z)) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin3D-2",
        source: "Makin3D-2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((-((z.z * z.z)) + ((z.y * z.x) + (z.y * z.x))) + (*aux).const_c.z);
		z.y = (-(((z.y * z.y) + ((z.z * z.x) + (z.z * z.x)))) + (*aux).const_c.y);
		z.x = (((z.x * z.x) + ((z.z * z.y) + (z.z * z.y))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin3D-3-4",
        source: "Makin3D-3-4.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Fuzzy-Y", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Fuzzy-Z", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Limiter", kind: ParamKind::Float, offset: 2, default: &[0.01] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((z.x * z.z) + (z.x * z.z)) * (1.0 + (-((z.y * z.y)) / (((z.x * z.x) + (z.z * z.z)) + abs(__MB2P1__))))) + (*aux).const_c.z);
		z.y = ((((z.x * z.y) + (z.x * z.y)) * (1.0 + (-((z.z * z.z)) / (((z.x * z.x) + (z.y * z.y)) + abs(__MB2P1__))))) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) - (z.y * z.y)) - (z.z * z.z)) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin4D p2b",
        source: "Makin4D p2b.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.x = ((*aux).const_c.x + z.x);
		z.y = ((*aux).const_c.y + z.y);
		z.z = ((*aux).const_c.z + z.z);
		z.w = (((*aux).const_c.w + __MB2P0__) + z.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin4D p4b",
        source: "Makin4D p4b.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.x = ((*aux).const_c.x + z.x);
		z.y = ((*aux).const_c.y + z.y);
		z.z = ((*aux).const_c.z + z.z);
		z.w = (((*aux).const_c.w + __MB2P0__) + z.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Makin4D p8b",
        source: "Makin4D p8b.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "xy mul", kind: ParamKind::Float, offset: 1, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.y = (((z.x * z.y) + (z.x * z.y)) + ((z.x * z.y) + (z.x * z.y)));
		z.x = (((z.x * z.x) - (z.y * z.y)) + ((z.x * z.x) - (z.y * z.y)));
		z.x = (z.x * (-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0));
		z.y = ((-(((((z.x * z.x) + (z.y * z.y)) + ((z.z * z.z) + (z.w * z.w))) / sqrt(((z.x * z.x) + (z.y * z.y))))) + 1.0) * z.y);
		z.w = (((z.x * z.w) + (z.y * z.z)) + ((z.x * z.w) + (z.y * z.z)));
		z.z = (((z.x * z.z) - (z.y * z.w)) + ((z.x * z.z) - (z.y * z.w)));
		z.x = ((*aux).const_c.x + z.x);
		z.y = ((*aux).const_c.y + z.y);
		z.z = ((*aux).const_c.z + z.z);
		z.w = (((*aux).const_c.w + __MB2P0__) + z.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "Mandel4DBiC",
        source: "Mandel4DBiC.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = ((((z.z * z.w) + (z.z * z.w)) + (*aux).const_c.w) + __MB2P0__);
		z.z = (((z.z * z.z) - (z.w * z.w)) + (*aux).const_c.z);
		z.y = (((z.x * z.y) + (z.x * z.y)) + (*aux).const_c.y);
		z.x = (((z.x * z.x) - (z.y * z.y)) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "MandelView",
        source: "MandelView.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Power", kind: ParamKind::Float, offset: 0, default: &[8.0] },
            GeneratedParam { path: "z-Mul", kind: ParamKind::Float, offset: 1, default: &[-1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t56: f32 = 0.0;
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t16 = cos((atan2(z.y, z.x) * __MB2P0__));
		t8 = sin((atan2(z.y, z.x) * __MB2P0__));
		t32 = cos((atan2(z.z, z.y) * __MB2P0__));
		t24 = sin((atan2(z.z, z.y) * __MB2P0__));
		t48 = cos((atan2(z.z, z.x) * __MB2P0__));
		t40 = sin((atan2(z.z, z.x) * __MB2P0__));
		t56 = exp2((((__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))))) % 1.0) + (__MB2P0__ * log2(abs(sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))))))));
		z.x = ((((t16 * t48) + -(((t24 * t40) * t8))) * t56) + (*aux).const_c.x);
		z.y = (((t8 * t32) * t56) + (*aux).const_c.y);
		z.z = ((((((t24 * t48) * t8) + (t16 * t40)) * t56) * __MB2P1__) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "MandyCousin",
        source: "MandyCousin.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Z mul 1", kind: ParamKind::Float, offset: 2, default: &[2.0] },
            GeneratedParam { path: "Z mul 2", kind: ParamKind::Float, offset: 3, default: &[2.0] },
            GeneratedParam { path: "W mul", kind: ParamKind::Float, offset: 4, default: &[2.0] },
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 5, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((((z.x * z.w) + (z.y * z.z)) * __MB2P4__) + (*aux).const_c.w) + __MB2P5__);
		z.z = ((((z.x * z.z) * __MB2P2__) + ((z.y * z.w) * __MB2P3__)) + (*aux).const_c.z);
		z.y = ((((z.x * z.y) + (z.z * z.w)) * __MB2P1__) + (*aux).const_c.y);
		z.x = ((((((z.x * z.x) + -((z.y * z.y))) + -((z.z * z.z))) + (z.w * z.w)) * __MB2P0__) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "MandyCousin2",
        source: "MandyCousin2.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Z mul 1", kind: ParamKind::Float, offset: 2, default: &[2.0] },
            GeneratedParam { path: "Z mul 2", kind: ParamKind::Float, offset: 3, default: &[-2.0] },
            GeneratedParam { path: "W mul", kind: ParamKind::Float, offset: 4, default: &[2.0] },
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 5, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((((z.x * z.w) + (z.y * z.z)) * __MB2P4__) + (*aux).const_c.w) + __MB2P5__);
		z.z = ((((z.x * z.z) * __MB2P2__) + ((z.y * z.w) * __MB2P3__)) + (*aux).const_c.z);
		z.y = ((((z.x * z.y) - (z.z * z.w)) * __MB2P1__) + (*aux).const_c.y);
		z.x = ((((((z.x * z.x) + -((z.y * z.y))) + -((z.z * z.z))) - (z.w * z.w)) * __MB2P0__) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "MandyCousin3",
        source: "MandyCousin3.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Z mul 1", kind: ParamKind::Float, offset: 2, default: &[2.0] },
            GeneratedParam { path: "Z mul 2", kind: ParamKind::Float, offset: 3, default: &[-2.0] },
            GeneratedParam { path: "W mul", kind: ParamKind::Float, offset: 4, default: &[2.0] },
            GeneratedParam { path: "W add", kind: ParamKind::Float, offset: 5, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((((z.x * z.w) - (z.y * z.z)) * __MB2P4__) + (*aux).const_c.w) + __MB2P5__);
		z.z = ((((z.x * z.z) * __MB2P2__) + ((z.y * z.w) * __MB2P3__)) + (*aux).const_c.z);
		z.y = ((((z.x * z.y) + (z.z * z.w)) * __MB2P1__) + (*aux).const_c.y);
		z.x = ((((((z.x * z.x) + -((z.y * z.y))) + (z.z * z.z)) + (z.w * z.w)) * __MB2P0__) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "MsltoeFoldQuat",
        source: "MsltoeFoldQuat.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((((z.x * z.x) - (z.y * z.y)) - (z.z * z.z)) + (*aux).const_c.x);
		z.y = (((z.y * z.x) + (z.y * z.x)) + (*aux).const_c.y);
		z.z = (((z.z * z.x) + (z.z * z.x)) + (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "PseudoXDB",
        source: "PseudoXDB.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Z multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((abs(z.z) * abs(z.z)) - ((z.x * z.x) + (z.y * z.y))) * __MB2P0__) + (*aux).const_c.z);
		z.y = ((((((abs(z.z) + abs(z.z)) / sqrt(((z.x * z.x) + (z.y * z.y)))) * z.x) * z.y) + ((((abs(z.z) + abs(z.z)) / sqrt(((z.x * z.x) + (z.y * z.y)))) * z.x) * z.y)) + (*aux).const_c.y);
		z.x = ((((z.x * z.x) - (z.y * z.y)) * ((abs(z.z) + abs(z.z)) / sqrt(((z.x * z.x) + (z.y * z.y))))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "QuadrayBrot",
        source: "QuadrayBrot.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((abs((((*aux).const_c.x - (*aux).const_c.y) - (*aux).const_c.z)) - (z.z * z.z)) + (z.w * z.w)) + ((z.x * z.y) + (z.x * z.y)));
		z.z = (((abs(((-((*aux).const_c.x) + (*aux).const_c.y) - (*aux).const_c.z)) + (z.z * z.z)) - (z.w * z.w)) + ((z.x * z.y) + (z.x * z.y)));
		z.y = (((abs(((-((*aux).const_c.x) - (*aux).const_c.y) + (*aux).const_c.z)) + (z.y * z.y)) - (z.x * z.x)) + ((z.w * z.z) + (z.w * z.z)));
		z.x = (((abs((((*aux).const_c.x + (*aux).const_c.y) + (*aux).const_c.z)) + (z.x * z.x)) - (z.y * z.y)) + ((z.w * z.z) + (z.w * z.z)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "QuickDudley",
        source: "QuickDudley.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((z.y * z.y) + ((z.z * z.x) + (z.z * z.x))) + (*aux).const_c.z);
		z.y = (((z.z * z.z) + ((z.y * z.x) + (z.y * z.x))) + (*aux).const_c.y);
		z.x = (((z.x * z.x) - ((z.z * z.y) + (z.z * z.y))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "QuickDudleyKM",
        source: "QuickDudleyKM.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((z.y * z.y) - ((z.z * z.x) + (z.z * z.x))) + (*aux).const_c.z);
		z.y = (((z.z * z.z) + ((z.y * z.x) + (z.y * z.x))) + (*aux).const_c.y);
		z.x = (((z.x * z.x) - ((z.z * z.y) + (z.z * z.y))) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "RuckerBrot1",
        source: "RuckerBrot1.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Y mul", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Z mul", kind: ParamKind::Float, offset: 2, default: &[-2.0] },
            GeneratedParam { path: "Cz mul", kind: ParamKind::Float, offset: 3, default: &[-1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((z.z * sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))))) * __MB2P2__) + (abs((*aux).const_c.z) * __MB2P3__));
		z.y = ((((((z.x * z.y) * abs(z.x)) / (((z.x * z.x) + (z.y * z.y)) + 1e-10)) * (((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))) / (sqrt(((z.x * z.x) + (z.z * z.z))) + 1e-10))) * __MB2P1__) + (*aux).const_c.y);
		z.x = (((((((z.x * z.x) + -((z.y * z.y))) / (((z.x * z.x) + (z.y * z.y)) + 1e-10)) * (((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))) / (sqrt(((z.x * z.x) + (z.z * z.z))) + 1e-10))) * abs(z.x)) * __MB2P0__) + (*aux).const_c.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Abs4d",
        source: "_Abs4d.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = abs(z.x);
		z.y = abs(z.y);
		z.z = abs(z.z);
		z.w = abs(z.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_AbsX",
        source: "_AbsX.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (abs((z.x + __MB2P0__)) + -(__MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_AbsY",
        source: "_AbsY.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (abs((z.y + __MB2P0__)) + -(__MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_AbsZ",
        source: "_AbsZ.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (abs((z.z + __MB2P0__)) + -(__MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_AmazingBox",
        source: "_AmazingBox.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Min R 1", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Min R 2", kind: ParamKind::Float, offset: 2, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (z.w * select(select(__MB2P0__, (__MB2P0__ / ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x))))), (1.0 >= ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))))), __MB2P1__, (((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))) < __MB2P2__)));
		z.z = ((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * select(select(__MB2P0__, (__MB2P0__ / ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x))))), (1.0 >= ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))))), __MB2P1__, (((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))) < __MB2P2__)));
		z.y = ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * select(select(__MB2P0__, (__MB2P0__ / ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x))))), (1.0 >= ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))))), __MB2P1__, (((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))) < __MB2P2__)));
		z.x = ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * select(select(__MB2P0__, (__MB2P0__ / ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x))))), (1.0 >= ((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))))), __MB2P1__, (((((abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z)) * (abs((z.z + 1.0)) - (abs((z.z - 1.0)) + z.z))) + ((abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)) * (abs((z.y + 1.0)) - (abs((z.y - 1.0)) + z.y)))) + ((abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)) * (abs((z.x + 1.0)) - (abs((z.x - 1.0)) + z.x)))) < __MB2P2__)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BRotateFromMag",
        source: "_BRotateFromMag.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (z.x * 0.7071067811865475);
		t16 = (z.y * 0.7071067811865475);
		t24 = (t8 + t16);
		z.y = (t16 - t8);
		z.x = ((t24 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.z = ((z.z * 0.816496580927726) + -((t24 * 0.5773502691896258)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BRotateToMag",
        source: "_BRotateToMag.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		z.z = ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726));
		z.x = (t8 - z.y);
		z.y = (t8 + z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BSkewXmaxV1",
        source: "_BSkewXmaxV1.m3f",
        param_floats: 9,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "xMin", kind: ParamKind::Float, offset: 0, default: &[0.707] },
            GeneratedParam { path: "xScale", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "xMax", kind: ParamKind::Float, offset: 2, default: &[1.414] },
            GeneratedParam { path: "yzMin", kind: ParamKind::Float, offset: 3, default: &[0.5] },
            GeneratedParam { path: "yzScale", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "yzMax", kind: ParamKind::Float, offset: 5, default: &[1.0] },
            GeneratedParam { path: "xPixelMin", kind: ParamKind::Float, offset: 6, default: &[0.707] },
            GeneratedParam { path: "xPixelScale", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "xPixelMax", kind: ParamKind::Float, offset: 8, default: &[1.414] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((abs(z.x) - __MB2P0__) * (-(abs(z.x)) + __MB2P2__)) * __MB2P1__);
		t16 = ((abs((*aux).const_c.x) * (abs((*aux).const_c.x) + __MB2P6__)) * __MB2P7__);
		z.x = (-((((sqrt(((z.y * z.y) + (z.z * z.z))) * __MB2P4__) + t8) + t16)) + z.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT1_4D_Transform1",
        source: "_BT1_4D_Transform1.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t24 = abs(((z.x * 0.5) + (z.w * 0.8660254037844387)));
		t16 = ((z.x * 0.8660254037844387) + -((z.w * 0.5)));
		t8 = (((((z.x * 0.8660254037844387) + -((z.w * 0.5))) * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = abs((t8 + (z.y * 0.7071067811865475)));
		z.x = abs((t8 + -((z.y * 0.7071067811865475))));
		z.z = abs(((t16 * 0.5773502691896258) + (z.z * 0.816496580927726)));
		t8 = ((z.x + z.y) * 0.7071067811865475);
		z.y = ((-(z.x) + z.y) * 0.7071067811865475);
		t16 = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.w = (-((((t8 * 0.816496580927726) + (z.z * 0.5773502691896258)) * 0.5)) + (t24 * 0.8660254037844387));
		z.x = ((((t16 * 0.8660254037844387) + (t24 * 0.5)) * __MB2P0__) - __MB2P1__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT1_4D_clampXYZ",
        source: "_BT1_4D_clampXYZ.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t32 = sqrt(((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))));
		t24 = abs(((z.x * 0.5) + (z.w * 0.8660254037844387)));
		t16 = ((z.x * 0.8660254037844387) + -((z.w * 0.5)));
		t8 = (((((z.x * 0.8660254037844387) + -((z.w * 0.5))) * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = abs((t8 + (z.y * 0.7071067811865475)));
		z.x = abs((t8 + -((z.y * 0.7071067811865475))));
		z.z = abs(((t16 * 0.5773502691896258) + (z.z * 0.816496580927726)));
		t8 = ((z.x + z.y) * 0.7071067811865475);
		z.y = ((-(z.x) + z.y) * 0.7071067811865475);
		t16 = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.w = (-((((t8 * 0.816496580927726) + (z.z * 0.5773502691896258)) * 0.5)) + (t24 * 0.8660254037844387));
		t8 = ((t16 * 0.8660254037844387) + (t24 * 0.5));
		t16 = (t32 / sqrt(((((t16 * 0.8660254037844387) + (t24 * 0.5)) * ((t16 * 0.8660254037844387) + (t24 * 0.5))) + ((z.y * z.y) + (z.z * z.z)))));
		z.x = ((((t32 / sqrt(((((t16 * 0.8660254037844387) + (t24 * 0.5)) * ((t16 * 0.8660254037844387) + (t24 * 0.5))) + ((z.y * z.y) + (z.z * z.z))))) * t8) * __MB2P0__) - __MB2P1__);
		z.y = ((z.y * __MB2P0__) * t16);
		z.z = ((z.z * __MB2P0__) * t16);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT1_Transform1",
        source: "_BT1_Transform1.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = abs((t8 + (z.y * 0.7071067811865475)));
		z.x = abs((t8 + -((z.y * 0.7071067811865475))));
		z.z = abs(((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		t8 = ((z.x + z.y) * 0.7071067811865475);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.y = ((-(z.x) + z.y) * 0.7071067811865475);
		z.x = ((z.x * __MB2P0__) - __MB2P1__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT2_Transform2",
        source: "_BT2_Transform2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((t8 + z.y) * (t8 + z.y));
		t8 = ((t8 - z.y) * (t8 - z.y));
		z.z = (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs((sqrt((t16 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs((sqrt((t8 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs((sqrt((t8 + t16)) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT3_Transform3",
        source: "_BT3_Transform3.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.7] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[1.7] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = abs((t8 + z.y));
		t8 = abs((t8 - z.y));
		z.z = abs(((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs(((t16 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs(((t8 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs(((t8 + t16) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT4_Transform4",
        source: "_BT4_Transform4.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((t8 + z.y) * (t8 + z.y));
		t8 = ((t8 - z.y) * (t8 - z.y));
		z.z = (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs(((t16 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs(((t8 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs(((t8 + t16) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BT5b_Transform5b",
        source: "_BT5b_Transform5b.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((((t8 + z.y) * (t8 + z.y)) * ((t8 + z.y) * (t8 + z.y))) * (((t8 + z.y) * (t8 + z.y)) * ((t8 + z.y) * (t8 + z.y))));
		t8 = ((((t8 - z.y) * (t8 - z.y)) * ((t8 - z.y) * (t8 - z.y))) * (((t8 - z.y) * (t8 - z.y)) * ((t8 - z.y) * (t8 - z.y))));
		z.z = (((((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726))) * (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)))) * ((((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726))) * (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)))));
		z.x = ((abs((sqrt(sqrt((t16 + z.z))) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs((sqrt(sqrt((t8 + z.z))) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs((sqrt(sqrt((t8 + t16))) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BenesiSpheric",
        source: "_BenesiSpheric.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[0.81649658092773] },
        ],
        derivations: &[],
        wgsl: r####"
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (z.y * z.y);
		t16 = (z.z * z.z);
		t24 = sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0));
		t24 = ((sqrt((select(((t8 + t16) / (z.x * z.x)), (1.0 / ((t8 + t16) / (z.x * z.x))), (1.0 > ((t8 + t16) / (z.x * z.x)))) + 1.0)) * __MB2P0__) * t24);
		t16 = (sqrt((select(((t8 + t16) / (z.x * z.x)), (1.0 / ((t8 + t16) / (z.x * z.x))), (1.0 > ((t8 + t16) / (z.x * z.x)))) + 1.0)) * __MB2P0__);
		z.z = (z.z * t24);
		z.y = (z.y * t24);
		z.x = (z.x * t16);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BenesiT1",
        source: "_BenesiT1.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = abs((t8 + (z.y * 0.7071067811865475)));
		z.x = abs((t8 + -((z.y * 0.7071067811865475))));
		z.z = abs(((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		t8 = ((z.x + z.y) * 0.7071067811865475);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
		z.y = ((-(z.x) + z.y) * 0.7071067811865475);
		z.x = ((z.x * __MB2P0__) - __MB2P1__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BenesiT2",
        source: "_BenesiT2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = ((t8 + z.y) * (t8 + z.y));
		t8 = ((t8 - z.y) * (t8 - z.y));
		z.z = (((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)) * ((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs((sqrt((t16 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs((sqrt((t8 + z.z)) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs((sqrt((t8 + t16)) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BenesiT3",
        source: "_BenesiT3.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.7] },
            GeneratedParam { path: "xOff", kind: ParamKind::Float, offset: 1, default: &[1.7] },
        ],
        derivations: &[],
        wgsl: r####"
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (((z.x * 0.816496580927726) + -((z.z * 0.5773502691896258))) * 0.7071067811865475);
		z.y = (z.y * 0.7071067811865475);
		t16 = abs((t8 + z.y));
		t8 = abs((t8 - z.y));
		z.z = abs(((z.x * 0.5773502691896258) + (z.z * 0.816496580927726)));
		z.x = ((abs(((t16 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.y = ((abs(((t8 + z.z) - __MB2P1__)) * __MB2P0__) * 0.7071067811865475);
		z.z = (abs(((t8 + t16) - __MB2P1__)) * __MB2P0__);
		z.y = (z.y - z.x);
		t8 = (z.y + z.x);
		z.z = (-((t8 * 0.5773502691896258)) + (z.z * 0.816496580927726));
		z.x = ((t8 * 0.816496580927726) + (z.z * 0.5773502691896258));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_BenesiUnSpheric",
        source: "_BenesiUnSpheric.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[0.81649658092773] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		var t8: f32 = 0.0;
		t8 = (z.y * z.y);
		t16 = (z.z * z.z);
		t8 = ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.y);
		t32 = (((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.y) * ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.y));
		t16 = ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z);
		t24 = ((1.0 / sqrt((select((((((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z) * ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z)) + t32) / (z.x * z.x)), (1.0 / (((((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z) * ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z)) + t32) / (z.x * z.x))), (1.0 > (((((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z) * ((1.0 / sqrt((select(((z.y * z.y) / (z.z * z.z)), (1.0 / ((z.y * z.y) / (z.z * z.z))), (1.0 > ((z.y * z.y) / (z.z * z.z)))) + 1.0))) * z.z)) + t32) / (z.x * z.x)))) + 1.0))) * __MB2P0__);
		z.z = (t16 * t24);
		z.y = (t8 * t24);
		z.x = (z.x * t24);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_CelticMode",
        source: "_CelticMode.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Good rune", kind: ParamKind::Float, offset: 0, default: &[-1.0] },
            GeneratedParam { path: "Evil rune", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mystic rune", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Heretic rune", kind: ParamKind::Float, offset: 3, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((abs((z.x + ((*aux).const_c.x * __MB2P0__))) + -(((*aux).const_c.x * __MB2P0__))) + (z.x * __MB2P1__));
		z.z = select(z.z, (abs((z.z + ((*aux).const_c.z * __MB2P2__))) + -(((*aux).const_c.z * __MB2P3__))), (0.0 != __MB2P2__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_DonutTransform",
        source: "_DonutTransform.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Offset", kind: ParamKind::Float, offset: 0, default: &[4.0] },
            GeneratedParam { path: "ROffset", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Order", kind: ParamKind::Float, offset: 2, default: &[5.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		t16 = (atan2(z.x, z.y) * __MB2P2__);
		t24 = sqrt(((z.x * z.x) + (z.y * z.y)));
		t32 = (sqrt(((z.x * z.x) + (z.y * z.y))) - __MB2P0__);
		z.y = ((cos(t16) * t24) - __MB2P1__);
		z.x = ((sin(t16) * t24) - __MB2P1__);
		z.w = (((z.z * z.z) - __MB2P1__) + (t32 * t32));
		z.z = (z.z - __MB2P1__);
		t16 = (atan2((*aux).const_c.x, (*aux).const_c.y) * __MB2P2__);
		t24 = sqrt((((*aux).const_c.x * (*aux).const_c.x) + ((*aux).const_c.y * (*aux).const_c.y)));
		t32 = (sqrt((((*aux).const_c.x * (*aux).const_c.x) + ((*aux).const_c.y * (*aux).const_c.y))) - __MB2P0__);
		(*aux).const_c.y = ((cos(t16) * t24) - __MB2P1__);
		(*aux).const_c.x = ((sin(t16) * t24) - __MB2P1__);
		(*aux).const_c.w = ((((*aux).const_c.z * (*aux).const_c.z) - __MB2P1__) + (t32 * t32));
		(*aux).const_c.z = ((*aux).const_c.z - __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Exp4D",
        source: "_Exp4D.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * cos(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))));
		z.y = (z.y * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
		z.z = (z.z * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
		z.w = (z.w * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Folding1",
        source: "_Folding1.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "R fold", kind: ParamKind::Float, offset: 0, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((abs((z.z + __MB2P0__)) - abs((z.z - __MB2P0__))) - z.z);
		z.y = ((abs((z.y + __MB2P0__)) - abs((z.y - __MB2P0__))) - z.y);
		z.x = ((abs((z.x + __MB2P0__)) - abs((z.x - __MB2P0__))) - z.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_FoldingInfy",
        source: "_FoldingInfy.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "x Fold", kind: ParamKind::Float, offset: 0, default: &[5.0] },
            GeneratedParam { path: "y Fold", kind: ParamKind::Float, offset: 1, default: &[5.0] },
            GeneratedParam { path: "z Fold", kind: ParamKind::Float, offset: 2, default: &[5.0] },
            GeneratedParam { path: "w Fold", kind: ParamKind::Float, offset: 3, default: &[5.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = select(z.w, (-((z.w - (round((z.w / ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__)))) * ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__))))) + (abs(((z.w - (round((z.w / ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__)))) * ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__)))) + __MB2P3__)) + -(abs(((z.w - (round((z.w / ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__)))) * ((__MB2P3__ + __MB2P3__) + (__MB2P3__ + __MB2P3__)))) - __MB2P3__))))), (__MB2P3__ != 0.0));
		z.z = select(z.z, (-((z.z - (round((z.z / ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__)))) * ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__))))) + (abs(((z.z - (round((z.z / ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__)))) * ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__)))) + __MB2P2__)) + -(abs(((z.z - (round((z.z / ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__)))) * ((__MB2P2__ + __MB2P2__) + (__MB2P2__ + __MB2P2__)))) - __MB2P2__))))), (__MB2P2__ != 0.0));
		z.y = select(z.y, (-((z.y - (round((z.y / ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__)))) * ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__))))) + (abs(((z.y - (round((z.y / ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__)))) * ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__)))) + __MB2P1__)) + -(abs(((z.y - (round((z.y / ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__)))) * ((__MB2P1__ + __MB2P1__) + (__MB2P1__ + __MB2P1__)))) - __MB2P1__))))), (__MB2P1__ != 0.0));
		z.x = select(z.x, (-((z.x - (round((z.x / ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__)))) * ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__))))) + (abs(((z.x - (round((z.x / ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__)))) * ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__)))) + __MB2P0__)) + -(abs(((z.x - (round((z.x / ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__)))) * ((__MB2P0__ + __MB2P0__) + (__MB2P0__ + __MB2P0__)))) - __MB2P0__))))), (__MB2P0__ != 0.0));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_FoldingOct",
        source: "_FoldingOct.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = select(select(select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))), select(select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))), (select(select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))) != select(select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y)))))));
		z.x = select(select(select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))), select(select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))), (select(select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))))) != select(select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))), select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y))), (select(abs(z.x), abs(z.y), (abs(z.x) != abs(z.y))) != select(abs(z.y), abs(z.x), (abs(z.x) != abs(z.y)))))));
		z.z = abs(z.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_FoldingVar",
        source: "_FoldingVar.m3f",
        param_floats: 12,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Fold X", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Fold xx", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Fold xy", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Fold xz", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Fold Y", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "Fold yx", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "Fold yy", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "Fold yz", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "Fold Z", kind: ParamKind::Float, offset: 8, default: &[1.0] },
            GeneratedParam { path: "Fold zx", kind: ParamKind::Float, offset: 9, default: &[0.0] },
            GeneratedParam { path: "Fold zy", kind: ParamKind::Float, offset: 10, default: &[0.0] },
            GeneratedParam { path: "Fold zz", kind: ParamKind::Float, offset: 11, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = abs((((z.y + (((abs((((((z.x * __MB2P5__) + (z.y * __MB2P6__)) + (z.z * __MB2P7__)) + __MB2P4__) + z.y)) - abs((z.y - ((((z.x * __MB2P5__) + (z.y * __MB2P6__)) + (z.z * __MB2P7__)) + __MB2P4__)))) - z.y) * __MB2P10__)) + ((z.z * __MB2P11__) + __MB2P8__)) - z.z));
		z.z = ((abs(((((abs((((((z.x * __MB2P1__) + (z.y * __MB2P2__)) + (z.z * __MB2P3__)) + __MB2P0__) + z.x)) - abs((z.x - ((((z.x * __MB2P1__) + (z.y * __MB2P2__)) + (z.z * __MB2P3__)) + __MB2P0__)))) - z.x) * __MB2P9__) + z.z)) - ((z.y + (((abs((((((z.x * __MB2P5__) + (z.y * __MB2P6__)) + (z.z * __MB2P7__)) + __MB2P4__) + z.y)) - abs((z.y - ((((z.x * __MB2P5__) + (z.y * __MB2P6__)) + (z.z * __MB2P7__)) + __MB2P4__)))) - z.y) * __MB2P10__)) + ((z.z * __MB2P11__) + __MB2P8__))) - z.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_HexaTransform",
        source: "_HexaTransform.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Offset", kind: ParamKind::Float, offset: 0, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (abs(((z.x * -0.5) - (z.y * 0.8660254037844386))) - __MB2P0__);
		z.z = (abs(z.z) - __MB2P0__);
		z.y = (abs(((z.y * 0.8660254037844386) + (z.x * -0.5))) - __MB2P0__);
		z.x = (abs(z.x) - __MB2P0__);
		(*aux).const_c.w = (abs((((*aux).const_c.x * -0.5) - ((*aux).const_c.y * 0.8660254037844386))) - __MB2P0__);
		(*aux).const_c.z = (abs((*aux).const_c.z) - __MB2P0__);
		(*aux).const_c.y = (abs((((*aux).const_c.y * 0.8660254037844386) + ((*aux).const_c.x * -0.5))) - __MB2P0__);
		(*aux).const_c.x = (abs((*aux).const_c.x) - __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_HopSqrtX",
        source: "_HopSqrtX.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((select(sqrt(abs(((z.x * __MB2P2__) + __MB2P0__))), -(sqrt(abs(((z.x * __MB2P2__) + __MB2P0__)))), (0.0 <= ((z.x * __MB2P2__) + __MB2P0__))) / __MB2P3__) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_HopSqrtY",
        source: "_HopSqrtY.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((select(sqrt(abs(((z.y * __MB2P2__) + __MB2P0__))), -(sqrt(abs(((z.y * __MB2P2__) + __MB2P0__)))), (0.0 <= ((z.y * __MB2P2__) + __MB2P0__))) / __MB2P3__) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_HopSqrtZ",
        source: "_HopSqrtZ.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((select(sqrt(abs(((z.z * __MB2P2__) + __MB2P0__))), -(sqrt(abs(((z.z * __MB2P2__) + __MB2P0__)))), (0.0 <= ((z.z * __MB2P2__) + __MB2P0__))) / __MB2P3__) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_LinCombineCxyz",
        source: "_LinCombineCxyz.m3f",
        param_floats: 9,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "CXx mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "CXy mul", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "CXz mul", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "CYx mul", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "CYy mul", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "CYz mul", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "CZx mul", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "CZy mul", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "CZz mul", kind: ParamKind::Float, offset: 8, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).const_c.x = ((((*aux).const_c.x * __MB2P0__) + ((*aux).const_c.y * __MB2P1__)) + ((*aux).const_c.z * __MB2P2__));
		(*aux).const_c.y = ((((*aux).const_c.x * __MB2P3__) + ((*aux).const_c.y * __MB2P4__)) + ((*aux).const_c.z * __MB2P5__));
		(*aux).const_c.z = ((((*aux).const_c.x * __MB2P6__) + ((*aux).const_c.y * __MB2P7__)) + ((*aux).const_c.z * __MB2P8__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_LinCombineXYZ",
        source: "_LinCombineXYZ.m3f",
        param_floats: 9,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Xx mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Xy mul", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Xz mul", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Yx mul", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Yy mul", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "Yz mul", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "Zx mul", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "Zy mul", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "Zz mul", kind: ParamKind::Float, offset: 8, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__));
		z.y = (((z.x * __MB2P3__) + (z.y * __MB2P4__)) + (z.z * __MB2P5__));
		z.z = (((z.x * __MB2P6__) + (z.y * __MB2P7__)) + (z.z * __MB2P8__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_LogX",
        source: "_LogX.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.01] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((z.x * __MB2P2__) + __MB2P0__);
		z.x = (__MB2P3__ * log2((abs(((z.x * __MB2P2__) + __MB2P0__)) + abs(__MB2P1__))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_LogY",
        source: "_LogY.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.01] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((z.y * __MB2P2__) + __MB2P0__);
		z.y = (__MB2P3__ * log2((abs(((z.y * __MB2P2__) + __MB2P0__)) + abs(__MB2P1__))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_LogZ",
        source: "_LogZ.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.01] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((z.z * __MB2P2__) + __MB2P0__);
		z.z = (__MB2P3__ * log2((abs(((z.z * __MB2P2__) + __MB2P0__)) + abs(__MB2P1__))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_NeoSqrX",
        source: "_NeoSqrX.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((select((((z.x * __MB2P2__) + __MB2P0__) * (((z.x * __MB2P2__) + __MB2P0__) + -(__MB2P1__))), (((z.x * __MB2P2__) + __MB2P0__) * (-(((z.x * __MB2P2__) + __MB2P0__)) + __MB2P1__)), (0.0 <= ((z.x * __MB2P2__) + __MB2P0__))) / __MB2P3__) - __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_NeoSqrY",
        source: "_NeoSqrY.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((select((((z.y * __MB2P2__) + __MB2P0__) * (((z.y * __MB2P2__) + __MB2P0__) + -(__MB2P1__))), (((z.y * __MB2P2__) + __MB2P0__) * (-(((z.y * __MB2P2__) + __MB2P0__)) + __MB2P1__)), (0.0 <= ((z.y * __MB2P2__) + __MB2P0__))) / __MB2P3__) - __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_NeoSqrZ",
        source: "_NeoSqrZ.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixSq", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Div", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((select((((z.z * __MB2P2__) + __MB2P0__) * (((z.z * __MB2P2__) + __MB2P0__) + -(__MB2P1__))), (((z.z * __MB2P2__) + __MB2P0__) * (-(((z.z * __MB2P2__) + __MB2P0__)) + __MB2P1__)), (0.0 <= ((z.z * __MB2P2__) + __MB2P0__))) / __MB2P3__) - __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_PolyFolding",
        source: "_PolyFolding.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Order", kind: ParamKind::Float, offset: 0, default: &[5.0] },
            GeneratedParam { path: "Shift (deg)", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Shift x", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Shift y", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Skew (deg)", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((z.x * sin((((-(round((((atan2((z.y + __MB2P3__), (z.x + __MB2P2__)) + ((__MB2P4__ + __MB2P1__) * 0.017453293)) / 6.283185307) * __MB2P0__))) / __MB2P0__) * 6.283185307) + (__MB2P1__ * 0.017453293)))) + (z.y * cos((((-(round((((atan2((z.y + __MB2P3__), (z.x + __MB2P2__)) + ((__MB2P4__ + __MB2P1__) * 0.017453293)) / 6.283185307) * __MB2P0__))) / __MB2P0__) * 6.283185307) + (__MB2P1__ * 0.017453293)))));
		z.x = ((z.y * sin((((-(round((((atan2((z.y + __MB2P3__), (z.x + __MB2P2__)) + ((__MB2P4__ + __MB2P1__) * 0.017453293)) / 6.283185307) * __MB2P0__))) / __MB2P0__) * 6.283185307) + (__MB2P1__ * 0.017453293)))) - (z.x * cos((((-(round((((atan2((z.y + __MB2P3__), (z.x + __MB2P2__)) + ((__MB2P4__ + __MB2P1__) * 0.017453293)) / 6.283185307) * __MB2P0__))) / __MB2P0__) * 6.283185307) + (__MB2P1__ * 0.017453293)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_QuadrayTransform",
        source: "_QuadrayTransform.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((z.x + z.y) + z.z);
		z.y = ((z.z - z.x) - z.y);
		z.z = ((z.y - z.x) - z.z);
		z.w = ((z.x - z.y) - z.z);
		(*aux).const_c.x = (((*aux).const_c.x + (*aux).const_c.y) + (*aux).const_c.z);
		(*aux).const_c.y = (((*aux).const_c.z - (*aux).const_c.x) - (*aux).const_c.y);
		(*aux).const_c.z = (((*aux).const_c.y - (*aux).const_c.x) - (*aux).const_c.z);
		(*aux).const_c.w = (((*aux).const_c.x - (*aux).const_c.y) - (*aux).const_c.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_QuadrayTransform2",
        source: "_QuadrayTransform2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Offset", kind: ParamKind::Float, offset: 0, default: &[-1.35] },
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 1, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((abs(((z.x + z.y) + z.z)) * __MB2P1__) + __MB2P0__);
		z.y = ((abs(((z.z - z.x) - z.y)) * __MB2P1__) + __MB2P0__);
		z.z = ((abs(((z.y - z.x) - z.z)) * __MB2P1__) + __MB2P0__);
		z.w = ((abs(((z.x - z.y) - z.z)) * __MB2P1__) + __MB2P0__);
		(*aux).const_c.x = ((abs((((*aux).const_c.x + (*aux).const_c.y) + (*aux).const_c.z)) * __MB2P1__) + __MB2P0__);
		(*aux).const_c.y = ((abs((((*aux).const_c.z - (*aux).const_c.x) - (*aux).const_c.y)) * __MB2P1__) + __MB2P0__);
		(*aux).const_c.z = ((abs((((*aux).const_c.y - (*aux).const_c.x) - (*aux).const_c.z)) * __MB2P1__) + __MB2P0__);
		(*aux).const_c.w = (__MB2P0__ + (abs((((*aux).const_c.x - (*aux).const_c.y) - (*aux).const_c.z)) * __MB2P1__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_ScaleC4d",
        source: "_ScaleC4d.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Scale C", kind: ParamKind::Float, offset: 1, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x * __MB2P0__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
		z.w = (z.w * __MB2P0__);
		(*aux).const_c.x = ((*aux).const_c.x * __MB2P1__);
		(*aux).const_c.y = ((*aux).const_c.y * __MB2P1__);
		(*aux).const_c.z = ((*aux).const_c.z * __MB2P1__);
		(*aux).const_c.w = ((*aux).const_c.w * __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Scaling",
        source: "_Scaling.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x * __MB2P0__);
		z.y = (z.y * __MB2P0__);
		z.z = (z.z * __MB2P0__);
		z.w = (z.w * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Scalingplus",
        source: "_Scalingplus.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "x scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "y scale", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "z scale", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "w scale", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (z.w * __MB2P3__);
		z.z = (z.z * __MB2P2__);
		z.y = (z.y * __MB2P1__);
		z.x = (z.x * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SinePow2",
        source: "_SinePow2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 16.0,
        params: &[
            GeneratedParam { path: "Y multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Z multiplier", kind: ParamKind::Float, offset: 1, default: &[-1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P1__) * z.z) + ((sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P1__) * z.z));
		z.x = (((z.x * z.x) - (z.y * z.y)) * ((((z.x * z.x) + (z.y * z.y)) - (z.z * z.z)) / ((z.x * z.x) + (z.y * z.y))));
		z.y = (((((((z.x * z.x) + (z.y * z.y)) - (z.z * z.z)) / ((z.x * z.x) + (z.y * z.y))) * (z.y * z.x)) + (((((z.x * z.x) + (z.y * z.y)) - (z.z * z.z)) / ((z.x * z.x) + (z.y * z.y))) * (z.y * z.x))) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SinhX",
        source: "_SinhX.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[-2.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((z.x * __MB2P2__) + __MB2P0__);
		z.x = (__MB2P3__ * log2((abs(((z.x * __MB2P2__) + __MB2P0__)) + abs(__MB2P1__))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SinhY",
        source: "_SinhY.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[-2.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((__MB2P3__ * log2((((z.y * __MB2P2__) + __MB2P0__) + sqrt(((((z.y * __MB2P2__) + __MB2P0__) * ((z.y * __MB2P2__) + __MB2P0__)) + 1.0))))) - __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SinhZ",
        source: "_SinhZ.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "fixLg", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mul", kind: ParamKind::Float, offset: 2, default: &[-2.0] },
            GeneratedParam { path: "Base", kind: ParamKind::Float, offset: 3, default: &[0.367879441] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((__MB2P3__ * log2((((z.z * __MB2P2__) + __MB2P0__) + sqrt(((((z.z * __MB2P2__) + __MB2P0__) * ((z.z * __MB2P2__) + __MB2P0__)) + 1.0))))) - __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SphereInvC",
        source: "_SphereInvC.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).const_c.z = ((*aux).const_c.z * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + ((*aux).const_c.z * (*aux).const_c.z)))));
		(*aux).const_c.y = ((*aux).const_c.y * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + ((*aux).const_c.z * (*aux).const_c.z)))));
		(*aux).const_c.x = ((*aux).const_c.x * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + ((*aux).const_c.z * (*aux).const_c.z)))));
		z.z = (z.z * (1.0 / ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))));
		z.y = (z.y * (1.0 / ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))));
		z.x = (z.x * (1.0 / ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_SphereInvC4d",
        source: "_SphereInvC4d.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).const_c.w = ((*aux).const_c.w * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + (((*aux).const_c.z * (*aux).const_c.z) + ((*aux).const_c.w * (*aux).const_c.w))))));
		(*aux).const_c.z = ((*aux).const_c.z * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + (((*aux).const_c.z * (*aux).const_c.z) + ((*aux).const_c.w * (*aux).const_c.w))))));
		(*aux).const_c.y = ((*aux).const_c.y * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + (((*aux).const_c.z * (*aux).const_c.z) + ((*aux).const_c.w * (*aux).const_c.w))))));
		(*aux).const_c.x = ((*aux).const_c.x * (1.0 / (((*aux).const_c.x * (*aux).const_c.x) + (((*aux).const_c.y * (*aux).const_c.y) + (((*aux).const_c.z * (*aux).const_c.z) + ((*aux).const_c.w * (*aux).const_c.w))))));
		z.w = (z.w * (1.0 / ((z.x * z.x) + ((z.y * z.y) + ((z.z * z.z) + (z.w * z.w))))));
		z.z = (z.z * (1.0 / ((z.x * z.x) + ((z.y * z.y) + ((z.z * z.z) + (z.w * z.w))))));
		z.y = (z.y * (1.0 / ((z.x * z.x) + ((z.y * z.y) + ((z.z * z.z) + (z.w * z.w))))));
		z.x = (z.x * (1.0 / ((z.x * z.x) + ((z.y * z.y) + ((z.z * z.z) + (z.w * z.w))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Tanh4D",
        source: "_Tanh4D.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Mulx", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Muly", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Mulz", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Mulw", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Final Mul", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = ((((z.x * __MB2P3__) + (z.y * __MB2P2__)) + -((z.z * __MB2P1__))) + (z.w * __MB2P0__));
		z.z = ((((z.x * __MB2P2__) + -((z.y * __MB2P3__))) + (z.z * __MB2P0__)) + (z.w * __MB2P1__));
		z.y = ((((z.x * __MB2P1__) + (z.y * __MB2P0__)) + (z.z * __MB2P3__)) + -((z.w * __MB2P2__)));
		z.x = ((((z.x * __MB2P0__) + -((z.y * __MB2P1__))) + -((z.z * __MB2P2__))) + -((z.w * __MB2P3__)));
		z.x = ((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * cos(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))));
		z.y = (z.y * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
		z.z = (z.z * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
		z.w = (z.w * select(((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))), (((exp2(((z.x * 1.4426950408889634) % 1.0)) * exp(z.x)) * sin(sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))) / sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)))), (1e-200 != sqrt((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z))))));
		z.x = (((1.0 / ((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)) + ((z.x + 1.0) * (z.x + 1.0)))) * (((((z.x + 1.0) * (z.x + -1.0)) + (z.y * z.y)) + (z.z * z.z)) + (z.w * z.w))) * __MB2P4__);
		z.y = (((1.0 / ((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)) + ((z.x + 1.0) * (z.x + 1.0)))) * (((z.x + 1.0) * z.y) + -((z.y * (z.x + -1.0))))) * __MB2P4__);
		z.z = (((1.0 / ((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)) + ((z.x + 1.0) * (z.x + 1.0)))) * (((z.x + 1.0) * z.z) + -(((z.x + -1.0) * z.z)))) * __MB2P4__);
		z.w = (((1.0 / ((((z.w * z.w) + (z.y * z.y)) + (z.z * z.z)) + ((z.x + 1.0) * (z.x + 1.0)))) * (((z.x + 1.0) * z.w) + -(((z.x + -1.0) * z.w)))) * __MB2P4__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_Translate",
        source: "_Translate.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y add", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z add", kind: ParamKind::Float, offset: 2, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_TranslateC4d",
        source: "_TranslateC4d.m3f",
        param_floats: 8,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "x add", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "y add", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "z add", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "w add", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "Cx add", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "Cy add", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "Cz add", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "Cw add", kind: ParamKind::Float, offset: 7, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.w = (z.w + __MB2P3__);
		(*aux).const_c.x = ((*aux).const_c.x + __MB2P4__);
		(*aux).const_c.y = ((*aux).const_c.y + __MB2P5__);
		(*aux).const_c.z = ((*aux).const_c.z + __MB2P6__);
		(*aux).const_c.w = ((*aux).const_c.w + __MB2P7__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_YplusSinZ",
        source: "_YplusSinZ.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (z.y + sin(z.z));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_YplusSinZ2",
        source: "_YplusSinZ2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "h (phase)", kind: ParamKind::Float, offset: 0, default: &[3.0] },
            GeneratedParam { path: "k (ampli)", kind: ParamKind::Float, offset: 1, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((((((cos(((z.z * __MB2P0__) * 3.141592653589793)) * __MB2P1__) - __MB2P1__) + 2.0) * 0.5) * sin(z.z)) + z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_addC",
        source: "_addC.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Cx multiplier", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Cy multiplier", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Cz multiplier", kind: ParamKind::Float, offset: 2, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + ((*aux).const_c.x * __MB2P0__));
		z.y = (z.y + ((*aux).const_c.y * __MB2P1__));
		z.z = (z.z + ((*aux).const_c.z * __MB2P2__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_boxtiling",
        source: "_boxtiling.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "x size", kind: ParamKind::Float, offset: 0, default: &[5.0] },
            GeneratedParam { path: "y size", kind: ParamKind::Float, offset: 1, default: &[5.0] },
            GeneratedParam { path: "z size", kind: ParamKind::Float, offset: 2, default: &[5.0] },
            GeneratedParam { path: "w size", kind: ParamKind::Float, offset: 3, default: &[5.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = select(z.w, (z.w - (round((z.w / __MB2P3__)) * __MB2P3__)), (__MB2P3__ != 0.0));
		z.z = select(z.z, (z.z - (round((z.z / __MB2P2__)) * __MB2P2__)), (__MB2P2__ != 0.0));
		z.y = select(z.y, (z.y - (round((z.y / __MB2P1__)) * __MB2P1__)), (__MB2P1__ != 0.0));
		z.x = select(z.x, (z.x - (round((z.x / __MB2P0__)) * __MB2P0__)), (__MB2P0__ != 0.0));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_conj",
        source: "_conj.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = -(z.y);
		z.z = -(z.z);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_conj2D",
        source: "_conj2D.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = -(z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_conj4D",
        source: "_conj4D.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = -(z.y);
		z.z = -(z.z);
		z.w = -(z.w);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_cosmartin",
        source: "_cosmartin.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[0.75] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((__MB2P3__ - z.x) * __MB2P0__);
		z.x = ((z.y + (-(cos((z.x * __MB2P2__))) * __MB2P1__)) * __MB2P0__);
		z.w = ((__MB2P4__ - z.z) * __MB2P0__);
		z.z = ((z.w + (-(cos((z.z * __MB2P2__))) * __MB2P1__)) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_dynamic",
        source: "_dynamic.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[0.1] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[0.1] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[1.5] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[1.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (((z.z + (-(cos((z.x * __MB2P3__))) * __MB2P1__)) + (-(sin((z.y * __MB2P4__))) * __MB2P2__)) * __MB2P0__);
		z.y = (((z.y + (-(cos((z.z * __MB2P3__))) * __MB2P1__)) + (-(sin((z.x * __MB2P4__))) * __MB2P2__)) * __MB2P0__);
		z.x = (((z.x + (-(cos((z.y * __MB2P3__))) * __MB2P1__)) + (-(sin((z.z * __MB2P4__))) * __MB2P2__)) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_gnarl2D",
        source: "_gnarl2D.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.03] },
            GeneratedParam { path: "X step", kind: ParamKind::Float, offset: 1, default: &[0.1] },
            GeneratedParam { path: "Y step", kind: ParamKind::Float, offset: 2, default: &[0.1] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 3, default: &[3.0] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 4, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((-((sin((sin(((sin((z.x * __MB2P4__)) + z.x) * __MB2P3__)) + z.x)) * __MB2P2__)) + z.y) * __MB2P0__);
		z.x = ((-((sin((sin(((sin((z.y * __MB2P4__)) + z.y) * __MB2P3__)) + z.y)) * __MB2P1__)) + z.x) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_gnarl2D_2",
        source: "_gnarl2D_2.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "X step", kind: ParamKind::Float, offset: 1, default: &[-0.1] },
            GeneratedParam { path: "Y step", kind: ParamKind::Float, offset: 2, default: &[-0.1] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 3, default: &[2.0] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 4, default: &[-4.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 5, default: &[-0.1] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (((sin((sin(((z.y + (cos(z.y) * __MB2P4__)) * __MB2P3__)) + ((z.x - z.y) * __MB2P5__))) * __MB2P1__) + z.x) * __MB2P0__);
		z.x = (((sin((((z.y + z.x) * __MB2P5__) - (sin((z.x + (cos(z.x) * __MB2P4__))) * __MB2P3__))) * __MB2P1__) + z.y) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_gnarl3D",
        source: "_gnarl3D.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.03] },
            GeneratedParam { path: "X step", kind: ParamKind::Float, offset: 1, default: &[0.1] },
            GeneratedParam { path: "Y step", kind: ParamKind::Float, offset: 2, default: &[0.1] },
            GeneratedParam { path: "Z step", kind: ParamKind::Float, offset: 3, default: &[0.1] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 4, default: &[3.0] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 5, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((-((sin((sin(((sin((z.y * __MB2P5__)) + z.y) * __MB2P4__)) + z.y)) * __MB2P3__)) + z.z) * __MB2P0__);
		z.y = ((-((sin((sin(((sin((z.x * __MB2P5__)) + z.x) * __MB2P4__)) + z.x)) * __MB2P2__)) + z.y) * __MB2P0__);
		z.x = ((-((sin((sin(((sin((z.z * __MB2P5__)) + z.z) * __MB2P4__)) + z.z)) * __MB2P1__)) + z.x) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_hopalm2",
        source: "_hopalm2.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[-0.4] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[0.1] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[0.1] },
            GeneratedParam { path: "Epsilon", kind: ParamKind::Float, offset: 5, default: &[0.2] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((__MB2P1__ * z.y) + -((z.x * __MB2P2__))) * __MB2P5__) + z.z);
		z.y = ((__MB2P1__ - z.x) * __MB2P0__);
		z.x = (atan2((((z.x * __MB2P2__) + -((z.z * __MB2P3__))) + __MB2P4__), 1.0) + z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_hopalong",
        source: "_hopalong.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[-0.4] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[0.1] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[0.1] },
            GeneratedParam { path: "Epsilon", kind: ParamKind::Float, offset: 5, default: &[0.2] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((__MB2P1__ * z.y) + -((z.x * __MB2P2__))) * __MB2P5__) + z.z);
		z.y = ((__MB2P1__ - z.x) * __MB2P0__);
		z.x = (select(sqrt(abs((((z.x * __MB2P2__) + -((z.z * __MB2P3__))) + __MB2P4__))), -(sqrt(abs((((z.x * __MB2P2__) + -((z.z * __MB2P3__))) + __MB2P4__)))), (0.0 <= (((z.x * __MB2P2__) + -((z.z * __MB2P3__))) + __MB2P4__))) + z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_hyc3Dinv",
        source: "_hyc3Dinv.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.z = (((((((z.y * z.y) - (z.x * z.z)) / -((((z.x * z.z) * z.y) - ((((((z.x * z.x) * z.x) + ((z.y * z.y) * z.y)) + ((z.z * z.z) * z.z)) - ((z.x * z.z) * z.y)) - ((z.x * z.z) * z.y))))) * __MB2P3__) - __MB2P1__) * __MB2P3__) - __MB2P2__);
		z.y = (((((z.x * z.x) - (z.y * z.z)) / -((((z.x * z.z) * z.y) - ((((((z.x * z.x) * z.x) + ((z.y * z.y) * z.y)) + ((z.z * z.z) * z.z)) - ((z.x * z.z) * z.y)) - ((z.x * z.z) * z.y))))) * __MB2P3__) - __MB2P0__);
		z.x = (((z.z * z.z) - (z.x * z.y)) / -((((z.x * z.z) * z.y) - ((((((z.x * z.x) * z.x) + ((z.y * z.y) * z.y)) + ((z.z * z.z) * z.z)) - ((z.x * z.z) * z.y)) - ((z.x * z.z) * z.y)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_invbisp",
        source: "_invbisp.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		t16 = ((z.x * z.x) + ((z.y * z.y) + (z.z * z.z)));
		t24 = sqrt(abs((((((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))) + (__MB2P0__ * __MB2P0__)) * (((z.x * z.x) + ((z.y * z.y) + (z.z * z.z))) + (__MB2P0__ * __MB2P0__))) + -((((__MB2P0__ * z.z) + (__MB2P0__ * z.z)) * ((__MB2P0__ * z.z) + (__MB2P0__ * z.z)))))));
		t32 = ((__MB2P0__ * z.z) + (__MB2P0__ * z.z));
		t16 = (t16 + -((__MB2P0__ * __MB2P0__)));
		z.z = atan2(z.y, z.x);
		z.x = (1.5707963267948966 - atan2(sqrt(((t24 * t24) - (t16 * t16))), t16));
		z.y = (1.4426950408889634 * log2(((t32 / t24) + sqrt((((t32 / t24) * (t32 / t24)) + 1.0)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_invcylindrical",
        source: "_invcylindrical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (cos(z.y) * z.x);
		z.y = (sin(z.y) * z.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_invspherical",
        source: "_invspherical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (sin(z.z) * z.x);
		z.y = (cos(z.y) * (((cos(z.z) * cos(z.y)) * z.x) * z.x));
		z.x = sin(z.y);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_invtorical",
        source: "_invtorical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 0, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (abs((-(z.x) + __MB2P0__)) * abs(cos(z.y)));
		z.y = ((abs((-(z.x) + __MB2P0__)) * abs(cos(z.y))) * tan(z.y));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_invtorical2",
        source: "_invtorical2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 1, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		var t48: f32 = 0.0;
		var t40: f32 = 0.0;
		var t32: f32 = 0.0;
		var t24: f32 = 0.0;
		var t16: f32 = 0.0;
		t16 = cos((z.y + __MB2P1__));
		t24 = sin((z.y + __MB2P1__));
		t32 = cos(z.z);
		t40 = sin(z.z);
		t48 = ((z.x * t16) + __MB2P0__);
		z.z = (z.x * t24);
		z.y = (t48 * t40);
		z.x = (t48 * t32);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_kamtor",
        source: "_kamtor.m3f",
        param_floats: 14,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Soft1", kind: ParamKind::Float, offset: 0, default: &[1.05] },
            GeneratedParam { path: "Soft2", kind: ParamKind::Float, offset: 1, default: &[-0.05] },
            GeneratedParam { path: "A1", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "B1", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "C1", kind: ParamKind::Float, offset: 4, default: &[0.0] },
            GeneratedParam { path: "D1", kind: ParamKind::Float, offset: 5, default: &[0.0] },
            GeneratedParam { path: "E1", kind: ParamKind::Float, offset: 6, default: &[0.0] },
            GeneratedParam { path: "F1", kind: ParamKind::Float, offset: 7, default: &[0.0] },
            GeneratedParam { path: "A2", kind: ParamKind::Float, offset: 8, default: &[0.0] },
            GeneratedParam { path: "B2", kind: ParamKind::Float, offset: 9, default: &[-1.0] },
            GeneratedParam { path: "C2", kind: ParamKind::Float, offset: 10, default: &[1.0] },
            GeneratedParam { path: "D2", kind: ParamKind::Float, offset: 11, default: &[0.0] },
            GeneratedParam { path: "E2", kind: ParamKind::Float, offset: 12, default: &[0.0] },
            GeneratedParam { path: "F2", kind: ParamKind::Float, offset: 13, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P1__) + (z.z * __MB2P0__));
		z.y = ((((((((z.x * __MB2P2__) + (z.y * __MB2P3__)) + ((z.x * z.x) * __MB2P4__)) + ((z.y * z.y) * __MB2P5__)) + ((z.x * z.y) * __MB2P6__)) + (sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P7__)) * select(sqrt((-(((1.0 / z.z) * (1.0 / z.z))) + 1.0)), sqrt((-((z.z * z.z)) + 1.0)), (1.0 <= abs(z.z)))) + (((((((z.x * __MB2P8__) + (z.y * __MB2P9__)) + ((z.x * z.x) * __MB2P10__)) + ((z.y * z.y) * __MB2P11__)) + ((z.x * z.y) * __MB2P12__)) + (sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P13__)) * select((1.0 / z.z), z.z, (1.0 <= abs(z.z)))));
		z.x = ((((((((z.x * __MB2P2__) + (z.y * __MB2P3__)) + ((z.x * z.x) * __MB2P4__)) + ((z.y * z.y) * __MB2P5__)) + ((z.x * z.y) * __MB2P6__)) + (sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P7__)) * select((1.0 / z.z), z.z, (1.0 <= abs(z.z)))) + (((((((z.x * __MB2P8__) + (z.y * __MB2P9__)) + ((z.x * z.x) * __MB2P10__)) + ((z.y * z.y) * __MB2P11__)) + ((z.x * z.y) * __MB2P12__)) + (sqrt(((z.x * z.x) + (z.y * z.y))) * __MB2P13__)) * select(sqrt((-(((1.0 / z.z) * (1.0 / z.z))) + 1.0)), sqrt((-((z.z * z.z)) + 1.0)), (1.0 <= abs(z.z)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_lorenz",
        source: "_lorenz.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[-2.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "r", kind: ParamKind::Float, offset: 4, default: &[-1.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((z.w * __MB2P4__) + -((z.y * z.z))) * __MB2P0__);
		z.z = (((z.x * z.y) + -((z.z * __MB2P2__))) * __MB2P0__);
		z.y = ((((z.x * __MB2P3__) - z.y) + -((z.x * z.z))) * __MB2P0__);
		z.x = ((((z.y - z.x) * __MB2P1__) + z.w) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_planeFold",
        source: "_planeFold.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Normal X", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Normal Y", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Normal Z", kind: ParamKind::Float, offset: 2, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x - (__MB2P0__ * ((((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)) + (((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)))));
		z.y = (z.y - (__MB2P1__ * ((((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)) + (((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)))));
		z.z = (z.z - (__MB2P2__ * ((((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)) + (((z.x * __MB2P0__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_poincare",
        source: "_poincare.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "P-Radius", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((abs(z.x) / abs((-((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) + __MB2P4__))) * __MB2P3__) - __MB2P0__);
		z.y = (((abs(z.y) / abs((-((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) + __MB2P4__))) * __MB2P3__) - __MB2P1__);
		z.z = (((abs(z.z) / abs((-((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) + __MB2P4__))) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_quadist",
        source: "_quadist.m3f",
        param_floats: 7,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Linear terms", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "a", kind: ParamKind::Float, offset: 1, default: &[0.25] },
            GeneratedParam { path: "b", kind: ParamKind::Float, offset: 2, default: &[-0.25] },
            GeneratedParam { path: "c", kind: ParamKind::Float, offset: 3, default: &[-0.25] },
            GeneratedParam { path: "d", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "e", kind: ParamKind::Float, offset: 5, default: &[0.25] },
            GeneratedParam { path: "f", kind: ParamKind::Float, offset: 6, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((z.z * __MB2P0__) + ((z.x * z.x) * (__MB2P5__ - __MB2P1__))) + ((z.y * z.y) * (__MB2P6__ - __MB2P2__))) + ((z.z * z.z) * (__MB2P4__ - __MB2P3__)));
		z.y = ((((z.y * __MB2P0__) + ((z.x * z.x) * (__MB2P6__ - __MB2P1__))) + ((z.y * z.y) * (__MB2P4__ - __MB2P2__))) + ((z.z * z.z) * (__MB2P5__ - __MB2P3__)));
		z.x = ((((z.x * __MB2P0__) + ((z.x * z.x) * (__MB2P4__ - __MB2P1__))) + ((z.y * z.y) * (__MB2P5__ - __MB2P2__))) + ((z.z * z.z) * (__MB2P6__ - __MB2P3__)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_rabinovich-f",
        source: "_rabinovich-f.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[3.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[0.87] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[1.1] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((((__MB2P1__ - __MB2P2__) * z.z) * (__MB2P4__ + (z.x * z.y))) * __MB2P0__);
		z.y = ((((((z.z * __MB2P2__) + __MB2P1__) + -((z.x * z.x))) * z.x) + (z.y * __MB2P3__)) * __MB2P0__);
		z.x = (((((z.z - __MB2P1__) + (z.x * z.x)) * z.y) + (z.x * __MB2P3__)) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalX",
        source: "_reciprocalX.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (1.0 / z.x);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalX2",
        source: "_reciprocalX2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter", kind: ParamKind::Float, offset: 0, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (1.0 / (abs(z.x) + __MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalX3",
        source: "_reciprocalX3.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter 1", kind: ParamKind::Float, offset: 0, default: &[0.5] },
            GeneratedParam { path: "Limiter 2", kind: ParamKind::Float, offset: 1, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (-((1.0 / (abs(z.x) + __MB2P0__))) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalX3b",
        source: "_reciprocalX3b.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter1 1", kind: ParamKind::Float, offset: 0, default: &[0.67] },
            GeneratedParam { path: "Limiter1 2", kind: ParamKind::Float, offset: 1, default: &[0.67] },
            GeneratedParam { path: "Limiter2 1", kind: ParamKind::Float, offset: 2, default: &[0.67] },
            GeneratedParam { path: "Limiter2 2", kind: ParamKind::Float, offset: 3, default: &[0.67] },
            GeneratedParam { path: "mul1", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "mul2", kind: ParamKind::Float, offset: 5, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = ((-((1.0 / (abs((z.x * __MB2P4__)) + __MB2P0__))) + __MB2P1__) + (-((1.0 / (abs(((z.x * z.x) * __MB2P5__)) + __MB2P2__))) + __MB2P3__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalY2",
        source: "_reciprocalY2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter", kind: ParamKind::Float, offset: 0, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (1.0 / (abs(z.y) + __MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalY3",
        source: "_reciprocalY3.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter 1", kind: ParamKind::Float, offset: 0, default: &[0.5] },
            GeneratedParam { path: "Limiter 2", kind: ParamKind::Float, offset: 1, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = (-((1.0 / (abs(z.y) + __MB2P0__))) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalY3b",
        source: "_reciprocalY3b.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter1 1", kind: ParamKind::Float, offset: 0, default: &[0.67] },
            GeneratedParam { path: "Limiter1 2", kind: ParamKind::Float, offset: 1, default: &[0.67] },
            GeneratedParam { path: "Limiter2 1", kind: ParamKind::Float, offset: 2, default: &[0.67] },
            GeneratedParam { path: "Limiter2 2", kind: ParamKind::Float, offset: 3, default: &[0.67] },
            GeneratedParam { path: "mul1", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "mul2", kind: ParamKind::Float, offset: 5, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = ((-((1.0 / (abs((z.y * __MB2P4__)) + __MB2P0__))) + __MB2P1__) + (-((1.0 / (abs(((z.y * z.y) * __MB2P5__)) + __MB2P2__))) + __MB2P3__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalZ2",
        source: "_reciprocalZ2.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter", kind: ParamKind::Float, offset: 0, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (1.0 / (abs(z.z) + __MB2P0__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalZ3",
        source: "_reciprocalZ3.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter 1", kind: ParamKind::Float, offset: 0, default: &[0.5] },
            GeneratedParam { path: "Limiter 2", kind: ParamKind::Float, offset: 1, default: &[0.5] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (-((1.0 / (abs(z.z) + __MB2P0__))) + __MB2P1__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_reciprocalZ3b",
        source: "_reciprocalZ3b.m3f",
        param_floats: 6,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limiter1 1", kind: ParamKind::Float, offset: 0, default: &[0.67] },
            GeneratedParam { path: "Limiter1 2", kind: ParamKind::Float, offset: 1, default: &[0.67] },
            GeneratedParam { path: "Limiter2 1", kind: ParamKind::Float, offset: 2, default: &[0.67] },
            GeneratedParam { path: "Limiter2 2", kind: ParamKind::Float, offset: 3, default: &[0.67] },
            GeneratedParam { path: "mul1", kind: ParamKind::Float, offset: 4, default: &[1.0] },
            GeneratedParam { path: "mul2", kind: ParamKind::Float, offset: 5, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = ((-((1.0 / (abs((z.z * __MB2P4__)) + __MB2P0__))) + __MB2P1__) + (-((1.0 / (abs(((z.z * z.z) * __MB2P5__)) + __MB2P2__))) + __MB2P3__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_rossler",
        source: "_rossler.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[0.5] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[1.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[1.5] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = ((-((__MB2P3__ * z.z)) + (__MB2P4__ * z.w)) * __MB2P0__);
		z.z = ((__MB2P2__ + (z.x * z.z)) * __MB2P0__);
		z.y = (((z.x + (z.y * __MB2P1__)) + z.w) * __MB2P0__);
		z.x = (-((z.y + z.z)) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_scaleC",
        source: "_scaleC.m3f",
        param_floats: 3,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "ScaleCx", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "ScaleCy", kind: ParamKind::Float, offset: 1, default: &[1.0] },
            GeneratedParam { path: "ScaleCz", kind: ParamKind::Float, offset: 2, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).const_c.x = ((*aux).const_c.x * __MB2P0__);
		(*aux).const_c.y = ((*aux).const_c.y * __MB2P1__);
		(*aux).const_c.z = ((*aux).const_c.z * __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_skew",
        source: "_skew.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Limit", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Skewness", kind: ParamKind::Float, offset: 1, default: &[-0.125] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = (z.z - (select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))) + abs(select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))))));
		z.y = (z.y - (select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))) + abs(select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))))));
		z.x = (z.x - (select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))) + abs(select((__MB2P0__ - (z.x * __MB2P1__)), select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))), (select((__MB2P0__ - (z.y * __MB2P1__)), (__MB2P0__ - (z.z * __MB2P1__)), ((__MB2P0__ - (z.z * __MB2P1__)) > (__MB2P0__ - (z.y * __MB2P1__)))) > (__MB2P0__ - (z.x * __MB2P1__)))))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_sphereXinv",
        source: "_sphereXinv.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = ((((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) / z.x) * __MB2P3__) - __MB2P0__);
		z.y = (((z.y / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P1__);
		z.z = (((z.z / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_sphereYinv",
        source: "_sphereYinv.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((z.x / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P0__);
		z.y = ((((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) / z.y) * __MB2P3__) - __MB2P1__);
		z.z = (((z.z / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_sphereZinv",
        source: "_sphereZinv.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((z.x / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P0__);
		z.y = (((z.y / (((z.x * z.x) + (z.y * z.y)) + (z.z * z.z))) * __MB2P3__) - __MB2P1__);
		z.z = ((((((z.x * z.x) + (z.y * z.y)) + (z.z * z.z)) / z.z) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_sqr_3D",
        source: "_sqr_3D.m3f",
        param_floats: 4,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 3, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (((z.x * z.x) * __MB2P0__) + __MB2P1__);
		z.y = (((z.y * z.y) * __MB2P0__) + __MB2P2__);
		z.z = (((z.z * z.z) * __MB2P0__) + __MB2P3__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_sqr_4D",
        source: "_sqr_4D.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Scale", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "fixX", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "fixY", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "fixZ", kind: ParamKind::Float, offset: 3, default: &[0.0] },
            GeneratedParam { path: "fixW", kind: ParamKind::Float, offset: 4, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (((z.x * z.x) * __MB2P0__) + __MB2P1__);
		z.y = (((z.y * z.y) * __MB2P0__) + __MB2P2__);
		z.z = (((z.z * z.z) * __MB2P0__) + __MB2P3__);
		z.w = (((z.w * z.w) * __MB2P0__) + __MB2P4__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_tocylindrical",
        source: "_tocylindrical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.y = atan2(z.y, z.x);
		z.x = sqrt(((z.x * z.x) + (z.y * z.y)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_toruspinv1",
        source: "_toruspinv1.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((abs(z.x) / sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) * __MB2P3__) - __MB2P0__);
		z.y = (((abs(z.y) / sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) * __MB2P3__) - __MB2P1__);
		z.z = (((abs(z.z) * sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) / __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_toruspinv2",
        source: "_toruspinv2.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((abs(z.x) / sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) * __MB2P3__) - __MB2P0__);
		z.y = (((abs(z.y) / sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) * __MB2P3__) - __MB2P1__);
		z.z = (((abs(z.z) * sqrt((((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z)))) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_toruspinv3",
        source: "_toruspinv3.m3f",
        param_floats: 5,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "X center", kind: ParamKind::Float, offset: 0, default: &[0.0] },
            GeneratedParam { path: "Y center", kind: ParamKind::Float, offset: 1, default: &[0.0] },
            GeneratedParam { path: "Z center", kind: ParamKind::Float, offset: 2, default: &[0.0] },
            GeneratedParam { path: "Radius", kind: ParamKind::Float, offset: 3, default: &[1.0] },
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 4, default: &[1.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.x = (z.x + __MB2P0__);
		z.y = (z.y + __MB2P1__);
		z.z = (z.z + __MB2P2__);
		z.x = (((abs(z.x) / (((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z))) * __MB2P3__) - __MB2P0__);
		z.y = (((abs(z.y) / (((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z))) * __MB2P3__) - __MB2P1__);
		z.z = (((abs(z.z) * (((-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__) * (-(((z.x * z.x) + (z.y * z.y))) + __MB2P4__)) + (z.z * z.z))) * __MB2P3__) - __MB2P2__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_tospherical",
        source: "_tospherical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
        ],
        derivations: &[],
        wgsl: r####"
		z.z = atan2(z.y, z.x);
		z.y = atan2(sqrt(((z.x * z.x) + (z.y * z.y))), z.z);
		z.x = sqrt(((z.z * z.z) + ((z.x * z.x) + (z.y * z.y))));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_totorical",
        source: "_totorical.m3f",
        param_floats: 1,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 0, default: &[2.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.y = atan2(z.y, z.x);
		z.x = (-(sqrt(((z.x * z.x) + (z.y * z.y)))) + __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_totorical2",
        source: "_totorical2.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "R", kind: ParamKind::Float, offset: 0, default: &[2.0] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 1, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		z.z = atan2(z.y, z.x);
		z.y = ((atan2((sqrt(((z.x * z.x) + (z.y * z.y))) - __MB2P0__), z.z) - __MB2P1__) - 1.5707963267948966);
		z.x = sqrt((((sqrt(((z.x * z.x) + (z.y * z.y))) - __MB2P0__) * (sqrt(((z.x * z.x) + (z.y * z.y))) - __MB2P0__)) + (z.z * z.z)));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_updateC",
        source: "_updateC.m3f",
        param_floats: 2,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "xyz mul", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "C-xyz mul", kind: ParamKind::Float, offset: 1, default: &[0.0] },
        ],
        derivations: &[],
        wgsl: r####"
		(*aux).const_c.x = ((z.x * __MB2P0__) + ((*aux).const_c.x * __MB2P1__));
		(*aux).const_c.y = ((z.y * __MB2P0__) + ((*aux).const_c.y * __MB2P1__));
		(*aux).const_c.z = ((z.z * __MB2P0__) + ((*aux).const_c.z * __MB2P1__));
	return z;
"####,
    },
    GeneratedFormula {
        name: "_vanderpol",
        source: "_vanderpol.m3f",
        param_floats: 8,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[2.0] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[0.6] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[0.1] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[0.4] },
            GeneratedParam { path: "Phi", kind: ParamKind::Float, offset: 5, default: &[2.0] },
            GeneratedParam { path: "r1", kind: ParamKind::Float, offset: 6, default: &[1.4] },
            GeneratedParam { path: "r2", kind: ParamKind::Float, offset: 7, default: &[1.4] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = (((z.x * __MB2P3__) + -((z.y * __MB2P4__))) + (((-((z.y * z.y)) * __MB2P7__) + __MB2P1__) * z.w));
		z.z = (((z.y * __MB2P3__) + -((z.x * __MB2P4__))) + (((-((z.x * z.x)) * __MB2P6__) + __MB2P5__) * z.z));
		z.y = (((z.y * __MB2P2__) + z.w) * __MB2P0__);
		z.x = (((z.x * __MB2P2__) + z.z) * __MB2P0__);
	return z;
"####,
    },
    GeneratedFormula {
        name: "_vanderpol2",
        source: "_vanderpol2.m3f",
        param_floats: 9,
        de_function: DeFunction::Delta,
        add_c: false,
        bailout: 1024.0,
        params: &[
            GeneratedParam { path: "Strength", kind: ParamKind::Float, offset: 0, default: &[1.0] },
            GeneratedParam { path: "Alpha", kind: ParamKind::Float, offset: 1, default: &[0.7] },
            GeneratedParam { path: "Beta", kind: ParamKind::Float, offset: 2, default: &[0.7] },
            GeneratedParam { path: "Gamma", kind: ParamKind::Float, offset: 3, default: &[0.7] },
            GeneratedParam { path: "Delta", kind: ParamKind::Float, offset: 4, default: &[0.2] },
            GeneratedParam { path: "Epsilon", kind: ParamKind::Float, offset: 5, default: &[0.2] },
            GeneratedParam { path: "Phi", kind: ParamKind::Float, offset: 6, default: &[0.2] },
            GeneratedParam { path: "Rho", kind: ParamKind::Float, offset: 7, default: &[0.8] },
            GeneratedParam { path: "r", kind: ParamKind::Float, offset: 8, default: &[1.4] },
        ],
        derivations: &[],
        wgsl: r####"
		z.w = ((((__MB2P6__ * z.z) * z.y) + -((__MB2P3__ * z.x))) + (((-((z.x * z.x)) * __MB2P8__) + __MB2P3__) * z.w));
		z.z = ((((__MB2P5__ * z.y) * z.w) + -((__MB2P2__ * z.x))) + (((-((z.x * z.x)) * __MB2P8__) + __MB2P2__) * z.z));
		z.y = ((((__MB2P4__ * z.z) * z.w) + -((__MB2P1__ * z.x))) + (((-((z.x * z.x)) * __MB2P8__) + __MB2P1__) * z.y));
		z.x = (((((z.x * __MB2P7__) + (z.y * __MB2P1__)) + (z.z * __MB2P2__)) + (z.w * __MB2P3__)) * __MB2P0__);
	return z;
"####,
    },
