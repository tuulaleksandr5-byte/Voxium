#!/usr/bin/env python3
"""Generate the original block Foley library, offline, with Python's standard library.

No recordings, downloaded samples or third-party libraries are used. The seed is stable;
rerunning this file reproduces every PCM sample. See docs/BLOCK_AUDIO.md.
"""
from __future__ import annotations

import argparse
import hashlib
import math
from pathlib import Path
import random
import struct
import wave

RATE = 22050
# Family, resonant body frequency, texture brightness, body/texture balance.
PROFILES = {
    "grass": ("rustle", 175, 1250, 0.24),
    "dirt": ("earth", 125, 850, 0.47),
    "stone": ("rock", 420, 1900, 0.76),
    "deepslate": ("rock", 245, 1250, 0.86),
    "sand": ("earth", 105, 1600, 0.20),
    "gravel": ("earth", 280, 2400, 0.54),
    "oak_log": ("wood", 185, 1150, 0.84),
    "leaves": ("rustle", 310, 2450, 0.09),
    "oak_planks": ("wood", 340, 1900, 0.88),
    "cobblestone": ("rock", 540, 2300, 0.66),
    "glass": ("glass", 1800, 3300, 0.60),
    "water": ("water", 380, 1700, 0.15),
    "bedrock": ("rock", 165, 900, 0.93),
    "snow": ("rustle", 110, 850, 0.16),
    "sandstone": ("rock", 350, 1350, 0.55),
}


def lowpass(values: list[float], cutoff: float) -> list[float]:
    alpha = 1 - math.exp(-2 * math.pi * cutoff / RATE)
    value = 0.0
    out = []
    for sample in values:
        value += alpha * (sample - value)
        out.append(value)
    return out


def synthesize(block: str, event: str, variant: int) -> list[int]:
    seed = int.from_bytes(hashlib.sha256(f"voxel-foley-v1/{block}/{event}/{variant}".encode()).digest()[:8], "little")
    rng = random.Random(seed)
    family, pitch, brightness, body_mix = PROFILES[block]
    pitch *= rng.uniform(0.91, 1.09)
    duration = {"break": 0.39, "step": 0.22, "place": 0.28}[event]
    duration *= rng.uniform(0.93, 1.10)
    if family in ("glass", "water"):
        duration *= 1.35
    count = int(duration * RATE)
    noise = [rng.uniform(-1, 1) for _ in range(count)]
    texture = lowpass(noise, brightness)
    bass = lowpass(noise, 230)
    texture = [a - 0.42 * b for a, b in zip(texture, bass)]
    # Secondary particles are irregularly spaced, giving breaks a scattered tail,
    # footsteps a compact heel/toe crunch, and placements one rounded impact.
    particles = {"break": 12, "step": 5, "place": 3}[event]
    strikes = [(0.0, 1.0)]
    for _ in range(particles):
        at = rng.uniform(0.008, duration * (0.7 if event == "break" else 0.42))
        strikes.append((at, rng.uniform(0.09, 0.33)))
    phases = [rng.uniform(0, 2 * math.pi) for _ in range(4)]
    out = []
    for i in range(count):
        t = i / RATE
        attack = 1 - math.exp(-t / (0.0035 if event == "step" else 0.002))
        decay = math.exp(-t / (duration * (0.24 if event == "place" else 0.34)))
        grains = sum(strength * math.exp(-(t - at) / 0.014)
                     for at, strength in strikes if t >= at)
        if family == "rustle":
            body = math.sin(2 * math.pi * pitch * t) * math.exp(-t / 0.027)
            sound = texture[i] * (0.45 * decay + 0.38 * grains) + body * body_mix * 0.17
        elif family == "earth":
            body = math.sin(2 * math.pi * pitch * t * (1 - 0.22 * t / duration)) * math.exp(-t / 0.034)
            sound = texture[i] * (0.37 * decay + 0.27 * grains) + body * body_mix * 0.34
        elif family in ("rock", "wood"):
            ratios = (1, 1.49, 2.17, 3.33) if family == "rock" else (1, 2.73, 4.18, 5.62)
            body = sum(math.sin(2 * math.pi * pitch * ratio * t + phases[j])
                       * math.exp(-t / (0.045 / (1 + j * 0.42))) / (1 + j * 1.5)
                       for j, ratio in enumerate(ratios))
            sound = body * body_mix * 0.35 + texture[i] * ((1 - body_mix) * decay + 0.18 * grains)
        elif family == "glass":
            body = sum(math.sin(2 * math.pi * pitch * ratio * t + phases[j])
                       * math.exp(-t / (0.11 / (1 + j * 0.25))) / (1 + j * 1.8)
                       for j, ratio in enumerate((1, 1.41, 1.93, 2.57)))
            sound = body * 0.15 + texture[i] * grains * 0.2
        else:  # Bubbly splashes: falling, inharmonic droplets over a soft wash.
            body = 0.0
            for j, (at, strength) in enumerate(strikes[:6]):
                age = t - at
                if age >= 0:
                    frequency = pitch * (1 + j * 0.38)
                    body += strength * math.sin(2 * math.pi * frequency * (age - age * age * 1.4)) * math.exp(-age / 0.045)
            sound = texture[i] * (decay * 0.56 + grains * 0.14) + body * 0.13
        fade = min(1.0, (duration - t) / 0.025)
        out.append(sound * attack * fade)
    # Consistent moderate peaks, with softer steps. Soft saturation catches rare
    # coincident grains without square-edged clipping or harsh, loud white noise.
    peak = max(abs(value) for value in out) or 1
    target = {"break": 0.51, "step": 0.32, "place": 0.42}[event]
    return [round(math.tanh(value / peak * 1.05) * target * 32767) for value in out]


def generate(root: Path) -> None:
    for block in PROFILES:
        directory = root / block
        directory.mkdir(parents=True, exist_ok=True)
        for event in ("break", "step", "place"):
            for variant in range(1, 4):
                samples = synthesize(block, event, variant)
                with wave.open(str(directory / f"{event}{variant}.wav"), "wb") as clip:
                    clip.setnchannels(1)
                    clip.setsampwidth(2)
                    clip.setframerate(RATE)
                    clip.writeframes(struct.pack(f"<{len(samples)}h", *samples))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path(__file__).resolve().parents[1] / "assets/sounds")
    args = parser.parse_args()
    generate(args.output)
    print(f"Generated {len(PROFILES) * 9} original WAV clips in {args.output}")
