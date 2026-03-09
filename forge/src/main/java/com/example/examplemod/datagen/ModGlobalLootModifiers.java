package com.example.examplemod.datagen;

import com.example.examplemod.item.ModItems;
import com.example.examplemod.loot.AddItemModifier;
import com.example.examplemod.loot.AddSusSandItemModifier;
import net.minecraft.data.PackOutput;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.storage.loot.predicates.LootItemBlockStatePropertyCondition;
import net.minecraft.world.level.storage.loot.predicates.LootItemCondition;
import net.minecraft.world.level.storage.loot.predicates.LootItemRandomChanceCondition;
import net.minecraftforge.common.data.GlobalLootModifierProvider;
import net.minecraftforge.common.loot.LootTableIdCondition;

public class ModGlobalLootModifiers extends GlobalLootModifierProvider {

  public ModGlobalLootModifiers(PackOutput output, String modid) {
    super(output, modid);
  }

  @Override
  protected void start() {

    add(
        "pine_cone_from_grass",
        new AddItemModifier(
            new LootItemCondition[] {
              LootItemBlockStatePropertyCondition.hasBlockStateProperties(Blocks.GRASS).build(),
              LootItemRandomChanceCondition.randomChance(0.999f).build()
            },
            ModItems.PINE_CONE.get()));

    add(
        "pine_cone_from_creeper",
        new AddItemModifier(
            new LootItemCondition[] {
              new LootTableIdCondition.Builder(ResourceLocation.parse("entities/creeper")).build()
            },
            ModItems.PINE_CONE.get()));

    add(
        "metal_detector_from_jungle_temples",
        new AddItemModifier(
            new LootItemCondition[] {
              new LootTableIdCondition.Builder(ResourceLocation.parse("chests/jungle_temple"))
                  .build()
            },
            ModItems.METAL_DETECTOR.get()));

    add(
        "metal_detector_from_suspicious_sand",
        new AddSusSandItemModifier(
            new LootItemCondition[] {
              new LootTableIdCondition.Builder(ResourceLocation.parse("archaeology/desert_pyramid"))
                  .build()
            },
            ModItems.METAL_DETECTOR.get()));
  }
}
