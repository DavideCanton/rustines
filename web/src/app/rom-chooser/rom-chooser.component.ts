import { Component, computed, inject, model, signal } from '@angular/core';
import { RomService } from '../rom.service';
import { Router } from '@angular/router';

@Component({
  imports: [],
  selector: 'rustines-rom-chooser',
  styleUrl: './rom-chooser.component.scss',
  templateUrl: './rom-chooser.component.html',
})
export class RomChooserComponent {
  protected zapper = signal(false);
  protected romName = signal<string | null>(null);
  protected error = signal<string | null>(null);

  protected romBytes: Uint8Array | null = null;

  protected romService = inject(RomService);
  protected router = inject(Router);

  async onFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    let file: File;

    if (input.files && input.files.length > 0) {
      file = input.files[0];
    } else {
      return;
    }

    if (!file) {
      return;
    }

    this.romName.set(file.name);

    try {
      this.romBytes = new Uint8Array(await file.arrayBuffer());
    } catch (error) {
      this.error.set(`${error}`);
    }
  }

  start() {
    this.romService.setValues({
      romName: this.romName()!,
      romBytes: this.romBytes!,
      zapper: this.zapper(),
    });
    this.router.navigateByUrl('/play');
  }
}
