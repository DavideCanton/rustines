import {
  Component,
  ElementRef,
  inject,
  OnInit,
  signal,
  viewChild,
} from '@angular/core';
import { RomService } from '../rom.service';
import { Router } from '@angular/router';
import init, { start_wasm } from '../generated/rustines_wasm';

const WASM_MODULE_PATH = '/generated/rustines_wasm_bg.wasm.gz';

@Component({
  imports: [],
  selector: 'rustines-play',
  styleUrl: './play.component.scss',
  templateUrl: './play.component.html',
})
export class PlayComponent implements OnInit {
  romName = signal<string | null>(null);
  error = signal(false);

  romService = inject(RomService);
  router = inject(Router);

  canvasContainer = viewChild<ElementRef<HTMLElement>>('canvasContainer');

  async ngOnInit() {
    const values = this.romService.values;

    if (!values) {
      this.router.navigateByUrl('/');
      return;
    }

    this.romName.set(values.romName);

    await init({ module_or_path: WASM_MODULE_PATH });
    start_wasm(values.romBytes, values.romName, values.zapper);

    const canvas: HTMLCanvasElement = this.canvasContainer()?.nativeElement
      .firstChild! as any;
    canvas.focus();
  }
}
