package com.example.examplemod.datagen;

import com.example.examplemod.MyMod;
import com.example.examplemod.item.ModItems;
import java.util.concurrent.CompletableFuture;
import javax.annotation.Nullable;
import net.minecraft.core.HolderLookup;
import net.minecraft.data.PackOutput;
import net.minecraft.data.tags.ItemTagsProvider;
import net.minecraft.tags.ItemTags;
import net.minecraft.world.level.block.Block;
import net.minecraftforge.common.data.ExistingFileHelper;

public class ModItemTags extends ItemTagsProvider {

  public ModItemTags(
      PackOutput pOutput,
      CompletableFuture<HolderLookup.Provider> a,
      CompletableFuture<TagLookup<Block>> b,
      @Nullable ExistingFileHelper existingFileHelper) {
    super(pOutput, a, b, MyMod.MODID, existingFileHelper);
  }

  @Override
  protected void addTags(HolderLookup.Provider provider) {

    this.tag(ItemTags.TRIMMABLE_ARMOR)
        .add(
            ModItems.SAPPHIRE_HELMET.get(),
            ModItems.SAPPHIRE_CHESTPLATE.get(),
            ModItems.SAPPHIRE_LEGGINGS.get(),
            ModItems.SAPPHIRE_BOOTS.get());

    this.tag(ItemTags.MUSIC_DISCS).add(ModItems.BAR_BRAWL_MUSIC_DISC.get());

    this.tag(ItemTags.CREEPER_DROP_MUSIC_DISCS).add(ModItems.BAR_BRAWL_MUSIC_DISC.get());
  }
}
