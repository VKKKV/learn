package com.example.examplemod.datagen;

import com.example.examplemod.datagen.loot.BlockLootTables;

import net.minecraft.data.PackOutput;
import net.minecraft.data.loot.LootTableProvider;
import net.minecraft.world.level.storage.loot.parameters.LootContextParamSets;

import java.util.List;
import java.util.Set;

public class LootTables extends LootTableProvider {
    public LootTables(PackOutput output) {
        super(
                output,
                Set.of(),
                List.of(
                        new LootTableProvider.SubProviderEntry(
                                BlockLootTables::new, LootContextParamSets.BLOCK)));
    }
}
