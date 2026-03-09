package com.example.examplemod.datagen;

import com.example.examplemod.MyMod;
import com.example.examplemod.block.ModBlocks;
import com.example.examplemod.util.ModTags;
import java.util.concurrent.CompletableFuture;
import net.minecraft.core.HolderLookup;
import net.minecraft.core.HolderLookup.Provider;
import net.minecraft.data.PackOutput;
import net.minecraft.tags.BlockTags;
import net.minecraftforge.common.Tags;
import net.minecraftforge.common.data.BlockTagsProvider;
import net.minecraftforge.common.data.ExistingFileHelper;

public class BlockTagsGenerator extends BlockTagsProvider {

  public BlockTagsGenerator(
      PackOutput output,
      CompletableFuture<Provider> lookupProvider,
      @org.jetbrains.annotations.Nullable ExistingFileHelper existingFileHelper) {
    super(output, lookupProvider, MyMod.MODID, existingFileHelper);
  }

  @Override
  protected void addTags(HolderLookup.Provider provider) {

    tag(ModTags.Blocks.METAL_DETECTOR_VALUABLES)
        .add(ModBlocks.SAPPHIRE_ORE.get())
        .addTag(Tags.Blocks.ORES);

    tag(BlockTags.MINEABLE_WITH_PICKAXE)
        .add(
            ModBlocks.SAPPHIRE_BLOCK.get(),
            ModBlocks.RAW_SAPPHIRE_BLOCK.get(),
            ModBlocks.NETHER_SAPPHIRE_ORE.get(),
            ModBlocks.DEEPSLATE_SAPPHIRE_ORE.get(),
            ModBlocks.SAPPHIRE_ORE.get(),
            ModBlocks.END_STONE_SAPPHIRE_ORE.get(),
            ModBlocks.SOUND_BLOCK.get(),
            ModBlocks.WTF.get());

    tag(ModTags.Blocks.NEEDS_SAPPHIRE_TOOL).add(ModBlocks.SOUND_BLOCK.get());
    tag(ModTags.Blocks.NEEDS_SAPPHIRE_TOOL).add(ModBlocks.WTF.get());

    tag(BlockTags.NEEDS_IRON_TOOL).add(ModBlocks.SAPPHIRE_BLOCK.get());

    tag(BlockTags.NEEDS_DIAMOND_TOOL).add(ModBlocks.RAW_SAPPHIRE_BLOCK.get());

    tag(BlockTags.NEEDS_DIAMOND_TOOL).add(ModBlocks.NETHER_SAPPHIRE_ORE.get());

    tag(Tags.Blocks.NEEDS_NETHERITE_TOOL).add(ModBlocks.END_STONE_SAPPHIRE_ORE.get());

    tag(BlockTags.FENCES).add(ModBlocks.SAPPHIRE_FENCE.get());
    tag(BlockTags.FENCE_GATES).add(ModBlocks.SAPPHIRE_FENCE_GATE.get());
    tag(BlockTags.WALLS).add(ModBlocks.SAPPHIRE_WALL.get());
  }
}
