import type { DataMeta } from '../data_types';

export class MetaInterpret {
	static getWorkerName = (worker_id: number, meta: DataMeta|undefined) => {
		if(meta){
			return MetaInterpret.getAllWorkers(meta)[worker_id];
		}
		else{
			return "Undefined metadata";
		}
	};

	static getAllWorkers = (meta: DataMeta) => {
		return meta.translators[1];
	};
}
