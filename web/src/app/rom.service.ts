import { Service } from '@angular/core';

interface Values {
  romName: string;
  romBytes: Uint8Array;
  zapper: boolean;
}

@Service()
export class RomService {
  #values: Values | null = null;

  get values(): Readonly<Values> | null {
    return this.#values;
  }

  setValues(values: Values) {
    this.#values = values;
  }
}
